# Findings: typed registration against a run-time binding table

Date: 2026-11-13. Round `202611130617-h6mb`, doc changelist item C10.

The round keeps registration typed and owes a choice it does not make: whether the contract should also carry a registration filled from data, on the erased, shape-described addressing chosen for resources on 2 July. The choice has two halves. Whether such a table can be built inside the stack's rules, and what it gives up in kind, is the sketch `mock/research/sketches/202611130728_data-bound-registration/`. What it costs to run is this note.

The short of it: a table that is consulted once a morsel costs nothing this bench can see, for columns and for units alike. A table consulted once a record costs seven to eight times the typed loop, and a stride that is only known at run time costs most of a second loop. So the measured cost does not decide between typed and data-bound registration. What decides is what the sketch found, which is about guarantees and about what the compiler can see through, and neither of those shows up in a timing of two unfused units.

The storage bench of June (`mock/research/202606210600_storage-bench-findings.md`) measured the erased addressing for resources and holds no number for columns. This is the column number.

## What was measured

The bench is `registration_binding` in `mock/benches/bench.toml`, run by the bench driver in `mock/benches/src/main.rs` through the mockspace bench harness: one dynamic library an arm, each in its own worker process, the timed call surrounded by the `realistic` workload program (scalar chains, pointer chases, a pass that evicts the first-level cache, branchy filler).

The workload is two units over five `u32` columns in 512-record morsels. The first unit writes `t` from `a` and `b`, the second writes `out` from `t` and `c`, each a short multiply and xor mix. Every pointer of every arm, the tables included, is cut from one bump arena, so a write to a column and a read of a table share a provenance and the compiler may not assume the table survived the write. The declarations the table arms read pass through `black_box`, standing in for a file. The kernels are in `mock/benches/support/registration-kernels/`, and their own tests hold all arms to one output. That comparison was shown failing by changing one arm's second unit, and the change was taken back.

| Arm | Units | Columns | Looked up |
|---|---|---|---|
| `reg_typed` | compiled into the loop | pointers held from the build | never |
| `reg_typed_twin` | the same arm again | the same | never |
| `reg_bound_columns` | compiled into the loop | from the table, shape checked | once a morsel |
| `reg_bound_units` | a function pointer from the table | from the table, shape checked | once a morsel |
| `reg_bound_strided` | a function pointer from the table | from the table, stepping by the declared stride | once a morsel |
| `reg_bound_record` | a function pointer from the table | from the table, shape checked | once a record |

The twin measures nothing of its own. The distance between it and the typed arm is what this host and this harness put between two runs of one program, and a difference between two other arms that is no larger than that is not one the bench has shown.

Profile `opt-level=3`, fat LTO, one codegen unit, target `x86_64` at its baseline feature set. Host: a cloud container on an Intel Xeon at 2.10 GHz, Linux 6.18, `rustc 1.98.0-nightly (57d06900f 2026-05-27)`. Eight passes of 2000 runs in batches of 100, which is 160 samples an arm, one cooldown cohort at 0 ms. Sources at commit `83f6f25`, which the run's metadata names as dirty for the untracked outputs of an earlier run beside it. The raw samples and the harness's own report for each size are `mock/benches/registration_binding_n<N>.csv`, `.meta.json` and `_findings.md`.

## The numbers

Medians of the timed call alone, in nanoseconds. N is the byte size of one column, so N / 4 records. In brackets, the paired difference to the typed arm with its 95% bootstrap interval, where the harness found the interval to exclude zero.

| N | records | typed | twin | bound columns | bound units | strided | per record |
|---|---|---|---|---|---|---|---|
| 4096 | 1024 | 925 | 903 | 854 | 872 | 2011 (+705, +76%) | 8384 (+7106, +769%) |
| 16384 | 4096 | 3492 | 3466 | 3445 | 3543 | 6280 (+2758, +79%) | 29309 (+25937, +743%) |
| 65536 | 16384 | 13641 | 13499 | 13834 (+542, +4.0%) | 13697 | 25231 (+11903, +87%) | 108212 (+93385, +685%) |
| 262144 | 65536 | 59608 | 60838 (+1883, +3.2%) | 59091 | 58753 | 104826 (+44832, +75%) | 451524 (+389769, +654%) |

The typed loop does both units in about 0.87 ns a record at every size, which is a vectorised loop, and a morsel of 512 records is about 440 ns of it.

The noise floor first. The twin sits 22, 26 and 142 ns under the typed arm at the three smaller sizes and 1883 ns over it at the largest, where the harness marks the difference as excluding zero before its correction for multiple comparisons and not after. Two builds of one source differ by up to 3.2% here.

The two arms that consult the table once a morsel are inside that floor at every size. `reg_bound_columns` is 34 ns under typed at the smallest size and 542 ns over it at 65536, both marked and both of the size the twin shows; at the other two sizes there is no difference to mark. `reg_bound_units` is never marked. Taking the upper end of each interval and dividing by the morsels in the run puts the cost of the table under 32 ns a morsel, for six checked lookups and, in the second of the two arms, two indirect calls. The twin's own upper ends come to 34 ns by the same division, so that figure is the resolution of the bench and not a measured cost.

A run-time stride costs 0.7 ns a record on top of the typed 0.87, at every size. The loop still walks contiguous `u32` values, since the declared stride is four bytes. What it loses is the compiler's knowledge of that, and with it the vector form.

A lookup and a call per record costs about 6 ns a record on top, at every size, or 3 ns a unit call. That is the cost of putting the table inside the record loop.

## What it decides

Where the table is consulted decides the cost, and what the table holds does not. A registration filled from data is affordable when binding happens at the morsel or above it, and with packed columns whose stride the body knows. Both conditions are design choices a data-bound registration could make, so cost does not rule it out.

It rules out two shapes. A data-declared unit that is called once a record is not a candidate at any size measured. A column whose stride is a run-time value pays 75% to 87% over the typed loop, so a data-declared column has to be packed, or its body has to be compiled for its stride.

It does not make data-bound registration free. The sketch's three limits stand: a data-declared column holds plain values only, a data-declared unit is opaque at its call, and the plan's refusals become run-time values. The bench has two units and neither arm fuses them, so the cost of the second limit is not in these numbers.

## What it does not decide

One workload, one host, one target feature set. The units are cheap and regular. A unit with a heavier body would hide the per-record call further, and a unit with a lighter one would show it more.

The typed arm here is two loops a morsel, the units unfused. The engine fuses a run of per-record units into one loop where it can, and a unit behind a pointer cannot join such a run. A bench of a fused typed fiber against the same units behind pointers would measure what data-declared units give up, and this one does not.

Binding is once a morsel. Once a frame or once a build would be cheaper still and was not measured, since once a morsel already measures as nothing.

Morsels are 512 records. At much smaller morsels the per-morsel lookups stop being small against the work, and the bench has no arm for that.

Nothing here runs through the engine. The arms are kernels in the shape of the two registrations, as the storage bench's were, and the engine's dispatch walk, its per-core programs and its plan recompute are not in the measured path.

The driver registers the bench as one whose arms must agree byte for byte on their eight-byte checksum of the `out` column, and then never checks it: it calls the harness's orchestrator, which by its own module documentation spawns workers and collects samples and does not run the validation pass. So no run of this bench compared the arms' outputs. That the checksum follows the input, and that the arms agree on the whole column, rests on the kernels' own tests alone.
