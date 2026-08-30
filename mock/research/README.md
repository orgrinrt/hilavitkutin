# Research

This directory splits at the seed consolidation of 2026-07-19.

`seed/` is the flat, self-contained expression of the effective canon: the
founding consolidation spec (`202603181200`, round `202604200055`) with the
full registered amendment chain (per addendum A2, `pre_seed/202607193200`)
applied once, superseded material left out. While unfrozen it is under
construction; when its manifest declares it frozen, all further canon changes
happen in the registry (`mock/registry/`), never here, and a registry row
outranks seed text on conflict.

`pre_seed/` is everything that came before: the amendment memos, roadmaps,
audits, expert panels, sketches, and notes that produced the seed. Read it as
history, not as canon; the surviving content is in `seed/` and, after the
drain, in the registry.

Everything new lands at this root, exactly as research always has: memos,
research rounds and their deliverables, expert panels, third-party reading,
prior art, citations, timestamped docs or timestamped subdirs, and sketches
under `sketches/`. What changes is its standing, not its shape. Nothing here
is ever mandatory reading: anything meaningful a piece of research produces
lands as registry rows (with the research cited as provenance via
`git::commit::<hash>` refs), and the research itself is the paper trail for
how those rows came to be. Nothing here is referenced as design authority.
