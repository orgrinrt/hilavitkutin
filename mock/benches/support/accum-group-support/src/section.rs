//! The name of a section, read back into its worker count and group width.

/// A section named `w<W>_g<G>`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Section {
    pub workers: u32,
    pub group:   u32,
}

/// Read a section name, the last `/` component of a bench name.
pub fn parse(bench_name: &str) -> Option<Section> {
    let section = bench_name.rsplit('/').next()?;
    let (w, g) = section.strip_prefix('w')?.split_once("_g")?;
    Some(Section {
        workers: w.parse().ok()?,
        group:   g.parse().ok()?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_section_reads_back_and_a_malformed_one_does_not() {
        assert_eq!(
            parse("accumulator_grouped/w4_g64"),
            Some(Section {
                workers: 4,
                group:   64,
            })
        );
        assert_eq!(parse("w1_g8").map(|s| s.group), Some(8));
        assert_eq!(parse("w4"), None);
        assert_eq!(parse("wx_g8"), None);
        assert_eq!(parse("fold_light_w4"), None);
    }
}
