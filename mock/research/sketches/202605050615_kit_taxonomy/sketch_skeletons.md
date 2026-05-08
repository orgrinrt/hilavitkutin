# Kit-shape skeletons for S5

The three kit shapes are described and analysed in `FINDINGS.md`. They
were not built as separate compile sketches because S4
(`../202605050555_replaceable_polarity/`) already exercises the same
three shapes via the `MockspaceKit` / `BenchTracingKit` / `LintPackKit`
type-name conventions. Re-reading S4's sketches with the S5 axis-mapping
question in mind is the empirical exercise.

The Owned-state declarations in S5 use illustrative `Resource<T>` /
`Column<T>` markers that are not present in S4's reduced surface; the
shape distinction (multi-marker, FFI-init, cooperative-public) is what
S5 evaluates, not the type-level encoding.
