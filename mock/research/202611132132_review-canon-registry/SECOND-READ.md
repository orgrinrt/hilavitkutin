# Second read: does a stamped provisional tenet bind, and who stamped it

This is the second, independent read that finding 4 of `REVIEW.md` calls for. It was done by an opus sub-agent dispatched by hv-canon-registry-2 on 13 November 2026, with a neutral brief: the question, the sources, and the ladder (op's words, then the rules written from them, then briefs and contracts, then agent output). It read without editing. Its answer is copied below with only its bold removed. Its quotations of op were checked against the captures by grep before this file was written.

## Question 1: does a stamped provisional tenet bind?

Answer: yes. It should read exactly as a provisional row reads: the best current statement, never cited as settled. `canon check` exits 0 with a note naming the stamp and its ticket. Making a stamped row bind nothing contradicts op's words, and those govern.

Op's words, the top of the ladder:

- `/home/user/.data/op-responses/202611022057_overnight-autonomy-and-the-provisional-stamp.md:25`, also quoted in `/home/user/.shared/rules/a-provisional-stamp-unblocks-work-while-op-is-away.md.tmpl:22`: "a temporary stamp that ratifies things ... Your convergence, or two domain experts after a panel stretch, can act as this stamp; and it's as good as mine. But needs my blessing down the line, it's never a final say, just a temporary one pending my approval, but still approves the work depending on it, so nothing gets blocked while I'm away."
- `/home/user/.data/op-responses/202611082159_op-on-im-off-to-sleep-keep-going-as-youve.md:25`: "Converged, provisional stamp as usual holds as ratified until reviewed by me."
- `/home/user/.data/op-responses/202611012029_op-hands-the-fix-epoch-over-for-the-night.md:29`: "two experts converging can fulfill a ratification in my place until I answer."
- `/home/user/.data/op-responses/202611122006_stamps-by-separate-experts-then-start-upstream-work.md:20`: "Use those as the stamps for the latter rounds with a bit of hedge for potential change if I overturn one."
- `/home/user/hilavitkutin/mock/research/202611130850_the-record-read-whole/OP-ANSWERS-18.md:5`: "The rest can be on provisional stamps to begin with". "The rest" is set against the rows he wants brought to him, the ones "clearly not backed by me or actively against the canon". So the 390 rows are not in the class held back from work.
- The limit he sets, `/home/user/.data/op-responses/202611121134_op-on-the-very-reason-we-have-the-mutations-and.md:24`: "stamping ratifications ornprovisionals that are clearly contradicting canon which is never allowed". A stamp cannot carry a row that contradicts canon. That class belongs with the rows brought to him, not with the stamped ones.

The rule written from those words, `a-provisional-stamp-unblocks-work-while-op-is-away.md.tmpl`:

- line 12: "That stamp is as good as op's for now, and the work depending on it proceeds."
- line 14: "A stamp is never the final say. It is provisional until he blesses it".

`/home/user/.shared/rules/autonomous-overnight-work.md.tmpl:9` says "Nothing produced autonomously ever becomes authority". The same line routes this case to the stamp rule: "Where a call op owes would stop the work, [[a-provisional-stamp-unblocks-work-while-op-is-away]]". So "never becomes authority" means a stamp never graduates to ratified. It does not mean the stamp blocks work.

Where coordinator and agent texts contradict op:

- `/home/user/job-hv-canon-registry-2.md:20` says "A row that is provisional, stamped or listed for op must not read as binding." For provisional and stamped rows this is wrong against the passages above. For rows listed for op it is right, since `OP-ANSWERS-18.md:5` says to bring those to him.
- The contract says "a stamp is never a ratification and the row keeps its standing". That part agrees with op. Its "Your non-binding half reads those two fields" does not: it makes the stamp fields the switch to not-binding.
- `mockspace-extra-lints` `acc10c2` implements the contradiction in three places:
  - in `registry/namespaces.toml`, the `standing` description says "a superseded or a stamped row does not bind whatever its standing";
  - in the same file, the `stamped_by` description says "binds nothing until the maintainer ratifies it";
  - the canon tool's `binds()` does the same: `!self.replaced() && !self.stamped() && self.standing.binds()`.
- The other helper's lint doc, `cbca5f0:src/lints/a_provisional_stamp_is_not_a_ratification.rs`, lines 1 to 9, agrees with op:
  - "the stamp is as good as his for now. It is never the final say";
  - "a row on a provisional stamp is never read as ratified";
  - "a stamp is beside the ratification ladder, not a rung on it".

  Not ratified is not the same as binding nothing, and the lint claims only the first.

Ambiguity:

- Op's words are not ambiguous on the operative point: the stamp "approves the work depending on it". The only tension is in wording, "holds as ratified" against "provisional" and "never a final say".
- Reading it as the provisional tier (binds, never cited as settled, carries its ticket) satisfies both. That is the tier the base `standing` description (`18ae1e9`) already gives `provisional`.
- The 13 November stamps came by a different route from his 2 November one. That is question 2, and it does not change whether they bind, because op himself said those rows "can be on provisional stamps".

What the coordinator has to decide, which is nothing about op's intent:

- correct brief line 20, and the contract's "non-binding half", for stamped rows;
- have `acc10c2` (both the namespace descriptions and `binds()`) changed so a stamp no longer makes a row unbound, agreed with arvo-registry, since the contract is shared;
- keep binding-nothing for the 106 `held` rows. On this branch they are `standing = "held"`, and op's 13 November and 12 November words support holding them.

## Question 2: does `stamped_by = "experts"`, or `coordinators`, describe the 390 rows truthfully?

Answer: neither does, and the shape has no value for this route.

All 390 carry `stamped_by = "experts"`. Their own `stamp` text says why: "nothing read in the written history shows it to be his or to be unbacked, and it stands on a stamp under the words of op 'The rest can be on provisional stamps to begin with' (OP-ANSWERS-18.md)". The authority is op's direct instruction, applied after a reading by agents. No coordinators converged, and no two experts agreed after a panel.

Passages:

- The two routes as op defined them (`202611022057...md:25`, rule line 12): "the three coordinators converge on it, or two domain experts agree after a panel stretch." Line 24 of the rule adds: "Converge means all three agree ... silence is not agreement."
- `acc10c2:registry/namespaces.toml`, the `stamped_by` description: "`coordinators` where the coordinators converged on it, `experts` where two domain experts agreed."
- The lint doc (`cbca5f0`, lines 3 to 7) names the same two routes: "`stamped_by`, which of the two routes stamped it".
- Op's instruction, `OP-ANSWERS-18.md:5`: "set a couple of sonnet agents going through the entire research and design ... The rest can be on provisional stamps to begin with." Line 14 of that file records "a couple" as provenance, not an intent, so it does not require two agreeing readers. No record shows that the 390 rows had two independent readings that agreed.
- The coordinator's own later call: "stamped_by says two experts or the coordinators decided the row; on the ~110 it would be a false record". The same reasoning applies to the 390. No expert decided their content; op decided their status in bulk.

So `experts` on these rows records a two-expert agreement that did not happen. `coordinators` would record a three-way convergence that also did not happen.

Ambiguity:

- Two sonnet readers could be called "two domain experts" if they really were separate seats and agreed per row, and "a couple" in his instruction leaves that open.
- The record read (the stamp text itself, `CLASS-SWEEP.md`, the research directory) describes a reading that found nothing either way, not an agreement on the row's content.
- The 2 November route also requires "after a panel stretch".

What the coordinator has to decide, one of these:

- Add a third `stamped_by` value naming op's own instruction, for example one meaning stamped on the maintainer's word: a stated bulk stamp with the passage cited.
  - Do it in `mockspace-extra-lints` with arvo-registry's agreement, since the values are fixed in the shared contract (`cbca5f0`: "a second spelling would be a second mechanism").
  - The arvo summary in `/home/user/.data/op-responses/202611131540_op-on-good-while-we-wait-why-dont-you-go.md:52` says arvo's 44 rows "stay at 'stated' on provisional stamps", so arvo likely has the same need.
- Or decide that op's own instruction is not a stamp in the 2 November sense. The 390 rows would then carry no `stamped_by` or `stamp` and stay plain `provisional`, with the reference to ticket `86aa80598714` moved to some other field.

Either way, the current `experts` value should not stand on these rows.
