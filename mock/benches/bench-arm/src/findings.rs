//! What a member does to the findings the harness writes before they are
//! committed.

/// Take the harness's editorialising and its emphasis out of a report.
///
/// Two things go. The `_Why it matters:_` lines are the harness telling a
/// reader which way to decide, which the numbers of these members do not
/// support, and disclaiming them in a preamble while leaving them in the body
/// reads worse than either. Bold spans go because the workspace writes no
/// inline styling in prose a human reads, and a generated body committed to
/// the repo is prose a human reads.
///
/// Everything else is left alone, including the figures and the highlight
/// headings the bold was wrapped around.
pub fn plain(report: &str) -> String {
    let mut out = String::with_capacity(report.len());
    for line in report.lines() {
        if line.trim_start().starts_with("_Why it matters:_") {
            continue;
        }
        out.push_str(&line.replace("**", ""));
        out.push('\n');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The stock line and the bold go; a figure, a heading, an italic that is
    /// not the stock line and the blank lines between stay as written.
    #[test]
    fn the_stock_lines_and_the_bold_go_and_the_rest_stays() {
        let report = "\
# element_wise

**Fastest:** `evs_std`, 1.30x ahead.
  _Why it matters:_ a concrete reason to switch.

| arm | median |
|---|---|
| evs_engine | 1.23 ms |

_n = 65536, warm_
";
        let want = "\
# element_wise

Fastest: `evs_std`, 1.30x ahead.

| arm | median |
|---|---|
| evs_engine | 1.23 ms |

_n = 65536, warm_
";
        assert_eq!(plain(report), want);
    }
}
