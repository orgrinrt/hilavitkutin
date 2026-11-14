# src changelist

**Round:** 202611132039-i9wf
**Phase:** IMPL
**Source topics:** `202611132039-f2df_topic.the-names-the-canon-retires.md`

The doc changelist gives four ratified tenets the names they retire and cites each tenet beside the template lines that state those names. Two lines of the `vocabulary-discipline` lint state `chain_group` to say it is dead, and this changelist cites the tenet beside them. Nothing the lint checks or reports changes but the punctuation of its message.

## Changes

### CHANGE 1: `mock/lints/vocabulary_discipline.rs`, the tenet beside `chain_group`

The module documentation's table of dead terms carries the tenet's address in an HTML comment in the `chain_group` row, which does not show in the rendered documentation, and the list of dead terms gains a line comment naming the tenet above its `chain_group` entry. Both lie within three lines of the name, which is the reach in which `a-retired-word-stays-out` reads a line as being about the retirement.

### CHANGE 2: the same file, two em-dashes

The module documentation and the message the lint prints each carried an em-dash, which the writing rules forbid. The documentation's becomes a colon and the message's a comma. The words of the message are otherwise as they were.
