//! The name of a section, read back into its workload and worker count.

/// A section named `<workload>_w<W>` or `<workload>_w<W>_<band>`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Section<'a> {
    /// The workload row's name, or `short` for the one-to-four-morsel band
    /// of `fold_light`.
    pub workload: &'a str,
    /// The worker count the section runs the arms at.
    pub workers:  u32,
}

/// Read a section name, the last `/` component of a bench name.
pub fn parse(bench_name: &str) -> Option<Section<'_>> {
    let section = bench_name.rsplit('/').next()?;
    let (workload, rest) = section.rsplit_once("_w")?;
    let workers = rest.split('_').next()?.parse().ok()?;
    Some(Section {
        workload,
        workers,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_section_reads_back_with_and_without_its_band() {
        assert_eq!(
            parse("single-trunk-convergence/fold_light_w4_big"),
            Some(Section {
                workload: "fold_light",
                workers:  4,
            })
        );
        assert_eq!(
            parse("append_w2"),
            Some(Section {
                workload: "append",
                workers:  2,
            })
        );
        assert_eq!(parse("fold_light"), None);
        assert_eq!(parse("fold_light_wx"), None);
    }
}
