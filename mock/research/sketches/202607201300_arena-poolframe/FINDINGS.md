# Findings: Arena Placement Dissolves Pin Only for a Fully Arena-Resident Plane

**Outcome: WORKS** (the hypothesis held exactly).

A worker-visible plane (pool sync words, per-worker contexts, the data
region) placed in a provider-style arena allocation, with the owning
handle holding only the plane pointer, runs 50 frames of the
seq/done/shutdown frame protocol across repeated MOVES of the handle
(the operation the shipped `Pin<&mut Self>` receiver forbids), with
byte-exact results and clean shutdown-join. No `PhantomPinned`, no
`Pin` receiver.

The load-bearing clause is "every byte a worker dereferences". The
shipped engine's workers hold a type-erased back-pointer to the WHOLE
scheduler (`WorkerCtx.sched`), not merely to the inline `PoolFrame`:
bindings, plan state, and the GATE-2 scratch are all reached through
it. Arena-placing the `PoolFrame` alone therefore dissolves nothing;
the Pin dissolves exactly when the full worker-visible plane (pool +
worker contexts + everything dispatch dereferences) relocates into the
arena and the scheduler shrinks to a movable handle around it. That is
the same relocation deviation 6 names for the inline GATE-2 scratch,
which is why the two deviations reconcile together or not at all.

Two incidental lessons: the `Send` wrapper must be captured wholesale
(edition-2021 disjoint closure capture will capture the raw-pointer
FIELD through a `wrapper.0` path and un-Send the closure; a method
accessor forces whole capture), and the frame protocol itself needed no
changes of any kind under the move, confirming the sync words care only
about their own addresses.

Next step unblocked: the deviation 1+6 evidence memo can now state the
canonical route's real shape (whole-plane arena residency plus a
movable handle) with a proven mechanism, and the bless-or-rebuild
choice prices that refactor against keeping the consumer-facing Pin.
