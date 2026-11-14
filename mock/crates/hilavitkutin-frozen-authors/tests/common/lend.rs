//! The eleven stores of the frozen authors lent to the reference runner, and
//! one frame of it read back.
//!
//! Macros, since the world's type is a nested pair eleven deep and is never
//! written out. Every name in them resolves where the macro is used, so a
//! crate that takes the frozen files by path uses them the same as one that
//! takes the crate.

/// Declare `$world`: the stores of the model's starting state, lent to a
/// reference runner. The live sides of the two buffered columns hold
/// `$poison`, which no frame may read.
#[macro_export]
macro_rules! lend {
    ($world:ident, $poison:expr) => {
        let mut start = Model::new();
        let mut heat_live = [Heat(Num::from_raw($poison)); RECORDS.0];
        let mut level_live = [Level(Num::from_raw($poison)); RECORDS.0];
        let $world = (
            Lent::<Column<Pos>>::over(&mut start.pos),
            (
                Lent::<Column<Vel>>::over(&mut start.vel),
                (
                    Lent::<Field>::over(&mut start.heat, &mut heat_live),
                    (
                        Lent::<Column<Edge>>::over(&mut start.edge),
                        (
                            Lent::<Accum<Crossed>>::over(&mut start.appended),
                            (
                                Lent::<Virtual<Overheat>>::unfired(),
                                (
                                    Lent::<Column<Alarms>>::over(&mut start.alarms),
                                    (
                                        Lent::<Pool>::over(&mut start.level, &mut level_live),
                                        (
                                            Lent::<Column<Peak>>::over(&mut start.peak),
                                            (
                                                Lent::<Column<Glow>>::over(&mut start.glow),
                                                (Lent::<Column<Ash>>::over(&mut start.ash), ()),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        );
    };
}

/// Hand what `$oracle` holds after a frame to `$record` as one
/// `record::State`, with `$rest` after it.
#[macro_export]
macro_rules! kept {
    ($oracle:expr, $record:path, $($rest:expr),+ $(,)?) => {
        $record(
            record::State {
                pos:      $oracle.of::<Column<Pos>, _>().values(),
                heat:     $oracle.of::<Field, _>().frozen(),
                alarms:   $oracle.of::<Column<Alarms>, _>().values(),
                level:    $oracle.of::<Pool, _>().frozen(),
                peak:     $oracle.of::<Column<Peak>, _>().values(),
                glow:     $oracle.of::<Column<Glow>, _>().values(),
                ash:      $oracle.of::<Column<Ash>, _>().values(),
                overheat: $oracle.of::<Virtual<Overheat>, _>().fired(),
                crossed:  $oracle.of::<Accum<Crossed>, _>().appended(),
            },
            $($rest),+
        )
    };
}
