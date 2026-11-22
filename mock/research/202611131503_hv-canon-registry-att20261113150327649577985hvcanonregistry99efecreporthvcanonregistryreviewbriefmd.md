Source: kaski channel (coord/kaski of clause-dev), att/20261113T150327649577985-hv-canon-registry-99efec/report-hv-canon-registry-review-brief.md

# Review brief: hilavitkutin, branch `work/canon-registry` against `dev`

Review locally. Post nothing to GitHub, push nothing, edit nothing. Return one
structured report as your final message, and write the same report to the
file named at the end.

## Where to work

The worktree `@@WORKTREE@@` holds the branch at `@@SHA@@`. Work there and in
no other checkout. Read other revisions out of the object store
(`git show <ref>:<path>`, `git diff <base>...<head>`); do not switch
branches.

    git -C @@WORKTREE@@ diff --stat origin/dev...@@SHA@@
    git -C @@WORKTREE@@ log --oneline origin/dev..@@SHA@@

## What the branch is

A helper job whose brief is `/home/user/job-hv-canon-registry.md`. Read the
brief whole first; it is what the work was asked to be. In short it asks for
four parts: the whole record of hilavitkutin read with a ledger of what was
read, a registry of typed rows under `mock/registry/` that transcribe what
the record says, a file `ANSWERS.md` answering eight questions from the
record with passages quoted, and query tools (part four: `cargo mock canon`,
`already-said` and `unearth`, taken from the shared lint pack by turning on
its `workflow` feature, with no tool copied into the repository).

What the branch changes:

- `mock/registry/` (subject, tenet and question rows) and the kinds for them
  in `mockspace.toml`, with `canon_paths` naming the subject and tenet rows.
- `mock/research/202611130850_the-record-read-whole/`: `LEDGER.md`,
  `ANSWERS.md`, `ROWS-FOR-OP.md`, `OP-ANSWERS.md`, `OP-ANSWERS-13.md`,
  `OP-ANSWERS-14.md`, `OP-ANSWERS-18.md`, `OP-ANSWERS-20.md`,
  `OP-ANSWERS-21.md`, `RECOVERED.md`, `IKIUNI.md`.
- The design-round files the repository's workflow required: rounds
  `202611130854` and `202611131444`.
- Two one-line fixes in `mock/lints/` for a changed lint API.
- A merge of `dev` into the branch (`f254a3d`), and after it the citations
  moved to where the merged files have their lines. Four rows whose
  passages the merge changed name a commit in `rev`.
- Part four, round `202611131444`: in `mockspace.toml` the pack's
  `workflow` feature, the pack's registry kinds and the tables of five
  tools; under `mock/registry/ticket/` and `mock/registry/plan/` the rows
  the pack starts from and eleven tasks filed by `cargo mock owed`; in the
  tenet rows, `rules_out` renamed `rejects` with `rules_out` left empty,
  the entries of `cites` moved to `related`, `subject` written beside `id`,
  and one row whose `what` now names two other rows by id; in source, the
  placeholder markers rewritten to cite those tasks and one comment
  reworded.

## The ladder of authority for this repository's record

1. The maintainer's own words (quoted in the `OP-ANSWERS*.md` files and the
   job brief).
2. The founding spec,
   `mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md`,
   with its amendments, and `mock/PRINCIPLES.md.tmpl`.
3. Everything else in the record (design rounds, research notes, sketches,
   bench findings) is agent-written intermediate work unless a passage says
   in words that it is the maintainer's ruling.

The registry rows are the repository's declared canon as of this branch. Their
existence, their place and their content are all challengeable, and so is
every `standing = "ratified"` on them (see the ratification pass below).
The shipped tree is a set of claims to test. `ikiuni` and `loimu` are
references and no part of hilavitkutin's record.

## Rules the work was held to

- A row never says more than its source passage says. Each row's `source`
  is `path:first-last`, and its `quote` field was filled from those lines by
  script.
- `origin = "stated"` only for the founding spec, the principles and passages
  whose text says they are the maintainer's ruling; everything else is
  `origin = "proposed"` with `written_by`.
- Where two texts state one thing two ways and neither says it replaces the
  other, both get a row and a `question` row names both.
- A claim about what the design says carries a file and line.
- Human-facing prose: no em-dashes or en-dashes, no bold or italic, no
  exclamation marks, no ASCII diagrams, the first sections are prose.
  (Quoted passages keep their source's characters.)
- Commit titles: `type: lowercase imperative`, under 72 characters, no
  parentheses, no trailing period, no bylines or tool adverts.
- No python. No lint severity lowered. No tool copied into the repository.

## The ratification pass

After the rows were written the maintainer said, verbatim
(`mock/research/202611130850_the-record-read-whole/OP-ANSWERS-18.md`):

> Also I am not about to answer hundreds of questions about canon rows to be
> ratified. Most of the designs and such are mine, and what isn't clearly my
> intent or come originally from me, likely has some written down prose
> ratifications or direct quotes from me. So what we do is you set a couple
> of sonnet agents going through the entire research and design as it is/was
> and you fill in what can be derived there as ratified or my call, fill
> them as ratified by default from written history. Only those that are
> clearly not backed by me or actively against the canon otherwise, you
> bring to me. The rest can be on provisional stamps to begin with

Readers then classed every row, and the classes were written in:

- A tenet with `standing = "ratified"` carries `ratified_by` and
  `ratified_from`. Each `ratified_from` entry is `<kind>: <path>:<first>-<last>`
  and names the passage held to show the row's content to be his: `founding`
  (the row's source is the founding spec or the principles), `ruled` (an
  amendment whose text says he ruled it), `quote` (a passage quoting him or
  saying in words that he ruled, chose or directed it), `founding-elsewhere`
  (the founding spec or the principles state the same point), and
  `ratified-elsewhere` (another passage says he ruled or directed this
  content). An entry led by `later` names a later passage of his on the
  same point.
- A tenet with a `stamp` and `standing = "provisional"` is one where the
  readers found neither his word nor anything against it.
- A tenet with neither is listed in `ROWS-FOR-OP.md` in the same directory,
  with a finding and options, for the maintainer.
- A question is `answered` with `answered_from` where the history was held
  to answer it in his words, open on a `stamp`, or listed in `ROWS-FOR-OP.md`.

The record's own order of authority is stated in
`mock/research/pre_seed/202607193200_canon-amendment-a2-precedence-and-rulings.md`
lines 10 to 41.

## What to review

1. The sweep first, from the worktree:
   `/home/user/.claude/skills/pr-review/pr-review all` (the branch against
   `dev`). Report its exit code and verdict lines.
2. `cargo mock` from the worktree root: report whether it passes.
3. The rows. They are too many to read all; sample across every file under
   `mock/registry/tenet/` and `mock/registry/question/`, at least sixty rows,
   chosen by you and spread over subjects and over both origins. For each,
   open the source file at the cited lines and say whether the row's `what`,
   `because`, `mechanism` and `rejects` hold to what the passage says,
   whether `origin` fits the convention above, and whether `cites` and
   `related` name rows that bear on it. List every row you checked.
3b. The ratification pass. Sample at least thirty ratified tenets spread
   over the five kinds and over the registry files, taking every kind other
   than `founding` more heavily. For each, open every `ratified_from`
   passage and say whether it shows what its kind requires for the whole of
   the row's `what`, or for part of it, or not. Sample at least ten stamped
   tenets and say for each whether anything you can find in the record
   shows it to be his or to be against a passage of his. Sample at least
   ten rows of `ROWS-FOR-OP.md` and say whether the finding follows from
   the passages it names and whether the options are real ones. Check
   every question that carries `answered_from`. List every row you checked
   with its verdict.
4. `ANSWERS.md`: for each of the eight answers, whether what it says the
   record decides and does not decide follows from the passages it quotes
   and the rows it cites, and whether any statement in it is made without a
   passage.
4b. Part four. Read the topic and the two changelists of round
   `202611131444` under `mock/design_rounds/202611131444/`. Say whether
   each claim in them holds on the branch: that no lint severity changed
   and no exemption was added in `mockspace.toml`; that every entry that
   was in `rules_out` is in `rejects` unchanged and every entry that was in
   `cites` is in `related` (compare against `c5f337a`); that the edits
   under `mock/crates` and `mock/benches` since `9594b53` are comments
   only; that the four rows carrying `rev` quote the lines of that commit.
   Say whether leaving `rules_out` empty on every row, and the one row
   whose `what` gained two row ids, hold to the rules above or not. Run
   `cargo mock canon find <a term>`, `cargo mock canon check <an id>`,
   `cargo mock already-said <a phrase>` and `cargo mock unearth <a subject>`
   and report what each answers.
5. `LEDGER.md`: whether its account of what was and was not read is
   consistent with itself and with the files on the branch.
6. Anything else on the branch you judge wrong, missing against the job
   brief, or out of place.

## Output

One report with these sections: sweep result; gate result; rows checked
(a table: id, verdict, and for each failing row the passage and what the row
says beyond or against it); answers; ledger; other findings; and a final
line that is exactly one of `verdict: clear`, `verdict: fix and re-review`,
`verdict: do not merge`. Number every finding. Give file and line for each.

Write the report to `@@REPORT@@` and return it as your final message.
