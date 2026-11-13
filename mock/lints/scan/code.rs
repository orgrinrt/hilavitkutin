//! Reading a Rust source line as code, for the repo lints that scan text.
//!
//! Shared by the lints that `#[path]` it in; it sits in a subdirectory,
//! which the lint discovery does not read, so it is no lint of its own.

/// The line with comments and string contents blanked out.
pub(crate) fn code_of(line: &str, in_block_comment: &mut bool) -> String {
    let bytes: Vec<char> = line.chars().collect();
    let mut out = String::with_capacity(line.len());
    let mut i = 0;
    let mut in_str = false;
    while i < bytes.len() {
        let c = bytes[i];
        let next = bytes.get(i + 1).copied();
        if *in_block_comment {
            if c == '*' && next == Some('/') {
                *in_block_comment = false;
                i += 2;
            } else {
                i += 1;
            }
            continue;
        }
        if in_str {
            if c == '\\' {
                i += 2;
                continue;
            }
            if c == '"' {
                in_str = false;
                out.push('"');
            }
            i += 1;
            continue;
        }
        // A char literal, so a quote inside one opens no string. A lifetime
        // (`'a`) has no closing quote two or more places on and is kept.
        if c == '\'' {
            let close = match next {
                Some('\\') => {
                    bytes[i + 2 ..]
                        .iter()
                        .position(|&x| x == '\'')
                        .map(|p| i + 2 + p)
                },
                Some(_) if bytes.get(i + 2) == Some(&'\'') => Some(i + 2),
                _ => None,
            };
            if let Some(end) = close {
                i = end + 1;
                continue;
            }
        }
        match (c, next) {
            ('/', Some('/')) => break,
            ('/', Some('*')) => {
                *in_block_comment = true;
                i += 2;
            },
            ('"', _) => {
                in_str = true;
                out.push('"');
                i += 1;
            },
            _ => {
                out.push(c);
                i += 1;
            },
        }
    }
    out
}

pub(crate) fn is_ident_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}
