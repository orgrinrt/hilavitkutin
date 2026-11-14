//! What the frozen lints share: which crates they read, how a file is walked
//! as code, and how a finding becomes a `LintError`.
//!
//! A frozen crate is written as a consumer would write it, against the api.
//! Each lint names one way a file could hide what the api makes its author
//! do, and refuses it in the `src/` of `hilavitkutin-frozen-authors`.
//!
//! Comments, doc comments and string literals are not read. There is no
//! escape: a frozen file that needs one is not written as a consumer could
//! write it, which is what the lints refuse. The runner drops a finding whose
//! line carries a `lint:allow` marker unless the lint says markers do not
//! decide, so each frozen lint says so (`Lint::markers_decide`).

use mockspace::{LintContext, LintError, Severity};

#[path = "../scan/code.rs"]
mod code;
pub(crate) use code::is_ident_char;

#[cfg(test)]
#[path = "runner.rs"]
pub(crate) mod runner;

/// The crates the frozen lints read.
pub(crate) fn applies_to(crate_name: &str) -> bool {
    crate_name.starts_with("hilavitkutin-frozen-")
}

/// One refused spot: the 1-based line and what is wrong there.
#[derive(Debug, PartialEq)]
pub(crate) struct Finding {
    pub(crate) line:    usize,
    pub(crate) message: String,
}

/// Each line of `text` as code, with its 1-based number.
fn code_lines(text: &str) -> impl Iterator<Item = (usize, String)> + '_ {
    let mut in_block_comment = false;
    text.lines()
        .enumerate()
        .map(move |(idx, raw)| (idx + 1, code::code_of(raw, &mut in_block_comment)))
}

/// Whether `text` holds `word` as a whole word: no identifier character
/// straight before it or straight after.
pub(crate) fn has_word(text: &str, word: &str) -> bool {
    text.match_indices(word).any(|(at, _)| {
        let before = text[.. at].chars().next_back();
        let after = text[at + word.len() ..].chars().next();
        !before.is_some_and(is_ident_char) && !after.is_some_and(is_ident_char)
    })
}

/// Each line of `text` whose code holds one of `words` as a whole word, with
/// the first of them it holds.
pub(crate) fn words<const N: usize>(
    text: &str,
    words: [&'static str; N],
) -> Vec<(usize, &'static str)> {
    code_lines(text)
        .filter_map(|(line, code)| {
            let word = words.into_iter().find(|word| has_word(&code, word))?;
            Some((line, word))
        })
        .collect()
}

/// Each attribute of `text`, inner and outer, from its `#` to the bracket
/// that closes it, with the 1-based line its `#` is on.
///
/// The attribute comes back with every whitespace character removed, so
/// `# ! [ feature` and `#![feature` read the same. The compiler takes an
/// attribute broken over lines between any two of its tokens, so the text is
/// read as one run of code and no attribute ends at the end of a line. One
/// that never closes runs to the end of the text.
pub(crate) fn attributes(text: &str) -> Vec<(usize, String)> {
    let code: Vec<(usize, char)> = code_lines(text)
        .flat_map(|(line, code)| {
            code.chars()
                .filter(|c| !c.is_whitespace())
                .map(|c| (line, c))
                .collect::<Vec<_>>()
        })
        .collect();
    let at = |i: usize| code.get(i).map(|&(_, c)| c);
    let mut found = Vec::new();
    let mut i = 0;
    while i < code.len() {
        let open = if at(i + 1) == Some('!') { i + 2 } else { i + 1 };
        if at(i) != Some('#') || at(open) != Some('[') {
            i += 1;
            continue;
        }
        let mut depth = 0usize;
        let mut end = code.len();
        for (k, &(_, c)) in code.iter().enumerate().skip(open) {
            match c {
                '[' => depth += 1,
                ']' if depth == 1 => {
                    end = k + 1;
                    break;
                },
                ']' => depth -= 1,
                _ => {},
            }
        }
        found.push((code[i].0, code[i .. end].iter().map(|&(_, c)| c).collect()));
        i = end;
    }
    found
}

/// Every finding `find` reports in the crate's sources, as hard errors.
pub(crate) fn check(
    ctx: &LintContext,
    name: &'static str,
    find: fn(&str) -> Vec<Finding>,
) -> Vec<LintError> {
    if !applies_to(ctx.crate_name) {
        return Vec::new();
    }
    let mut errors = Vec::new();
    for file in ctx.all_sources() {
        let path = file.rel_path.to_string_lossy().into_owned();
        for finding in find(file.text) {
            let mut e = LintError::with_severity(
                ctx.crate_name.to_string(),
                finding.line,
                name,
                finding.message,
                Severity::HARD_ERROR,
            );
            e.path = Some(path.clone());
            errors.push(e);
        }
    }
    errors
}
