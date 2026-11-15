# The Brief Every Slice Reader Was Given

Sibling of `AUDIT.md`. Each sub-agent that read a slice for this audit was told to read this file first. It is kept so a later reader can see what the slice readers were and were not asked to do. The slices themselves are listed in `LEDGER.md`.

You are a slice reader for an audit of hilavitkutin, a `no_std` morsel-driven columnar dataflow engine whose schedule is fixed at compile time. It sits on arvo (numerics) and notko (fallibility primitives, hlist). Its one known consumer is kaski, a game engine. You read and report. The judgement belongs to the auditor. Your final message is your digest and nothing else of yours is kept.

Hard rules:

- Read only, in every repository. Create, edit, commit and push nothing. No python. Spawn no agents. Run no bench unless your slice says so.
- The tree under audit is at `/home/user/.worktrees/side-audit-hilavitkutin`, commit `266650fb`. If your own working directory is not at that commit (`git log -1 --format=%h`), read through absolute paths under that directory.
- Shell commands with pipes or several operations are often refused in this container, and `ls` outside your working directory is refused. Use one simple command per call (`grep -rn`, `ls`, `wc -l`) and the Read tool, which takes absolute paths anywhere under `/home/user`. Read large files in big chunks.
- The canon is the registry declared at `mockspace.toml:14`. From the repository root, `cargo mock canon subjects`, `cargo mock canon subject <name>`, `cargo mock canon find <term>` and `cargo mock canon related <id>` query it.
- Every claim about what a document or the code says carries a repo-relative path and a line. Every claim about a measurement names the committed result file and the line. Quote decisive wording verbatim and briefly. Where you infer, say it is your inference. Where you did not read, say so.
- The earlier audit under `mock/research/202611131215_stack-audit/` and every other document are claims to test, not conclusions to inherit.
- Write plain prose with no bold, no italics and no em-dashes. Keep to the word limit your slice gives. End with a ledger: what you read whole, what in part, what not at all.
