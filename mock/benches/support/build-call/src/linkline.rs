//! The link line rustc prints with `--print link-args`, read back into a
//! command that can be run again.
//!
//! rustc prints the command as std's `Command` debug form: an optional
//! `cd "<dir>" &&`, `env -u <name>` for each variable taken out, `<NAME>="<value>"`
//! for each one set, then the program and every argument as a quoted string
//! with Rust's escapes.

use std::path::{Component, Path, PathBuf};
use std::process::Command;

/// One link command: where it runs, what it sets and takes out, and its words.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LinkLine {
    pub cwd:     Option<String>,
    pub removed: Vec<String>,
    pub set:     Vec<(String, String)>,
    pub argv:    Vec<String>,
}

impl LinkLine {
    /// The path after `-o`, the file the link writes.
    pub fn output(&self) -> Option<&str> {
        self.argv
            .iter()
            .position(|a| a == "-o")
            .and_then(|i| self.argv.get(i + 1))
            .map(String::as_str)
    }

    /// The file the link writes, held to `profile_dir`, the target directory
    /// of the profile the replay is for: the `-o` path, resolved against the
    /// line's working directory where it is relative, must lie under it with
    /// no `..` in it, or the line is another profile's or another cell's.
    pub fn output_under(&self, profile_dir: &Path) -> Result<PathBuf, String> {
        let out = self.output().ok_or("the link line has no `-o`")?;
        let out = match &self.cwd {
            Some(dir) if Path::new(out).is_relative() => Path::new(dir).join(out),
            _ => PathBuf::from(out),
        };
        let climbs = out.components().any(|c| c == Component::ParentDir);
        if climbs || !out.starts_with(profile_dir) {
            return Err(format!(
                "the link line writes {}, which is not under {}",
                out.display(),
                profile_dir.display()
            ));
        }
        Ok(out)
    }

    /// The command, ready to run.
    pub fn command(&self) -> Command {
        let mut c = Command::new(&self.argv[0]);
        c.args(&self.argv[1 ..]);
        if let Some(dir) = &self.cwd {
            c.current_dir(dir);
        }
        for k in &self.removed {
            c.env_remove(k);
        }
        for (k, v) in &self.set {
            c.env(k, v);
        }
        c
    }

    /// One field per line, for keeping it between calls.
    pub fn to_text(&self) -> String {
        let mut t = String::new();
        if let Some(d) = &self.cwd {
            t.push_str(&format!("cwd\t{d}\n"));
        }
        for k in &self.removed {
            t.push_str(&format!("unset\t{k}\n"));
        }
        for (k, v) in &self.set {
            t.push_str(&format!("set\t{k}\t{v}\n"));
        }
        for a in &self.argv {
            t.push_str(&format!("arg\t{a}\n"));
        }
        t
    }

    /// Read back what [`LinkLine::to_text`] wrote.
    pub fn from_text(text: &str) -> Option<LinkLine> {
        let mut l = LinkLine {
            cwd:     None,
            removed: Vec::new(),
            set:     Vec::new(),
            argv:    Vec::new(),
        };
        for line in text.lines() {
            let mut f = line.splitn(3, '\t');
            match (f.next()?, f.next()?, f.next()) {
                ("cwd", d, None) => l.cwd = Some(d.to_string()),
                ("unset", k, None) => l.removed.push(k.to_string()),
                ("set", k, Some(v)) => l.set.push((k.to_string(), v.to_string())),
                ("arg", a, None) => l.argv.push(a.to_string()),
                _ => return None,
            }
        }
        (!l.argv.is_empty()).then_some(l)
    }
}

/// The link line in rustc's output, the first line that has `"-o"` among its words.
pub fn find(stdout: &str) -> Option<LinkLine> {
    stdout
        .lines()
        .filter(|l| l.contains("\"-o\""))
        .find_map(|l| parse(l).ok())
}

/// One printed command.
pub fn parse(line: &str) -> Result<LinkLine, String> {
    let mut l = LinkLine {
        cwd:     None,
        removed: Vec::new(),
        set:     Vec::new(),
        argv:    Vec::new(),
    };
    let mut rest = line.trim();
    if let Some(r) = rest.strip_prefix("cd ") {
        let (dir, after) = quoted(r)?;
        l.cwd = Some(dir);
        rest = after
            .trim_start()
            .strip_prefix("&&")
            .ok_or("`cd` without `&&`")?
            .trim_start();
    }
    while let Some(r) = rest.strip_prefix("env -u ") {
        let end = r.find(' ').unwrap_or(r.len());
        l.removed.push(r[.. end].to_string());
        rest = r[end ..].trim_start();
    }
    while !rest.is_empty() {
        if rest.starts_with('"') {
            let (word, after) = quoted(rest)?;
            l.argv.push(word);
            rest = after.trim_start();
        } else {
            let eq = rest
                .find('=')
                .ok_or_else(|| format!("unquoted word: {rest:.40}"))?;
            if !l.argv.is_empty() {
                return Err(format!("a variable after the program: {rest:.40}"));
            }
            let key = rest[.. eq].to_string();
            let (value, after) = quoted(&rest[eq + 1 ..])?;
            l.set.push((key, value));
            rest = after.trim_start();
        }
    }
    if l.argv.is_empty() {
        return Err("no program".into());
    }
    Ok(l)
}

/// A string in Rust's debug quoting at the start of `s`, and what follows it.
fn quoted(s: &str) -> Result<(String, &str), String> {
    let body = s
        .strip_prefix('"')
        .ok_or_else(|| format!("expected a quote: {s:.40}"))?;
    let mut out = String::new();
    let mut chars = body.char_indices();
    while let Some((i, c)) = chars.next() {
        match c {
            '"' => return Ok((out, &body[i + 1 ..])),
            '\\' => {
                let (_, e) = chars.next().ok_or("a trailing backslash")?;
                match e {
                    '"' | '\\' | '\'' => out.push(e),
                    'n' => out.push('\n'),
                    't' => out.push('\t'),
                    'r' => out.push('\r'),
                    '0' => out.push('\0'),
                    'u' => {
                        let hex: String = chars
                            .by_ref()
                            .map(|(_, c)| c)
                            .skip_while(|c| *c == '{')
                            .take_while(|c| *c != '}')
                            .collect();
                        let v = u32::from_str_radix(&hex, 16)
                            .map_err(|e| format!("\\u{{{hex}}}: {e}"))?;
                        out.push(char::from_u32(v).ok_or_else(|| format!("\\u{{{hex}}}"))?);
                    },
                    other => return Err(format!("an escape rustc does not print: \\{other}")),
                }
            },
            c => out.push(c),
        }
    }
    Err("an unterminated quote".into())
}
