//! The runner against a model written by hand in plain integers.
//!
//! The model knows nothing of the api: it is the frame's meaning spelled out
//! over arrays. The runner holds when its stores equal the model's after
//! every frame. With the `reads-live` feature the runner reads the wrong side
//! of the buffered column and this test has to fail.

use arvo::{Bits, Bool, Hot, USize};
use hilavitkutin_api::store::{Accum, Buffered, Column, Exchange, Virtual};
use oracle_over_record_op::digest::Fold;
use oracle_over_record_op::units::{
    Alarm,
    Alarms,
    Cross,
    Crossed,
    Drift,
    Edge,
    Heat,
    Num,
    Overheat,
    Pos,
    Spread,
    Tally,
    Vel,
};
use oracle_over_record_op::{Lent, Oracle};

const WIDE: usize = 8;
const TALL: usize = 6;
const RECORDS: usize = WIDE * TALL;
const FRAMES: usize = 6;
const THRESHOLD: u64 = 40;
const LIMIT: u64 = 500;

fn num(value: u64) -> Num {
    Num::from_raw(value as _)
}

fn raw(value: Num) -> u64 {
    u64::from(value.to_raw())
}

/// The frame's meaning, over plain arrays.
struct Model {
    pos:     [u64; RECORDS],
    vel:     [u64; RECORDS],
    heat:    [u64; RECORDS],
    alarms:  [u64; RECORDS],
    crossed: Vec<u64>,
    fired:   bool,
}

impl Model {
    fn new() -> Self {
        let mut model = Model {
            pos:     [0; RECORDS],
            vel:     [0; RECORDS],
            heat:    [0; RECORDS],
            alarms:  [0; RECORDS],
            crossed: Vec::new(),
            fired:   false,
        };
        for i in 0 .. RECORDS {
            model.pos[i] = i as u64;
            model.vel[i] = (i % 5) as u64 + 1;
            model.heat[i] = ((i * 7) % 11) as u64;
        }
        model
    }

    fn frame(&mut self) {
        self.crossed.clear();
        // Drift writes the plain column in place, so Cross below reads what it wrote.
        for i in 0 .. RECORDS {
            self.pos[i] += self.vel[i];
        }
        // Spread reads the heat the frame began with at every record.
        let began = self.heat;
        let mut spread = [0; RECORDS];
        for i in 0 .. RECORDS {
            let east =
                if i % WIDE == WIDE - 1 { 0 } else { began.get(i + 1).copied().unwrap_or(0) };
            let west = if i % WIDE == 0 { 0 } else { began[i - 1] };
            let north = began.get(i + WIDE).copied().unwrap_or(0);
            let south = if i >= WIDE { began[i - WIDE] } else { 0 };
            spread[i] = began[i] + east + west + north + south;
        }
        for i in 0 .. RECORDS {
            let to = self.pos[i];
            let from = to - self.vel[i];
            if from < THRESHOLD && to >= THRESHOLD {
                self.crossed.push(to);
            }
        }
        // Alarm reads the buffered column too, so it sees the heat the frame began with.
        self.fired = began.iter().any(|heat| *heat >= LIMIT);
        if self.fired {
            for count in &mut self.alarms {
                *count += 1;
            }
        }
        // The boundary: what Spread wrote is what the next frame reads.
        self.heat = spread;
    }
}

#[test]
fn the_runner_equals_the_hand_model_after_every_frame() {
    let mut model = Model::new();

    let mut pos = [Pos(num(0)); RECORDS];
    let mut vel = [Vel(num(0)); RECORDS];
    let mut frozen = [Heat(num(0)); RECORDS];
    let mut live = [Heat(num(0)); RECORDS];
    let mut edge = [Edge {
        west: Bool(false),
        east: Bool(false),
    }; RECORDS];
    let mut crossed = [Crossed(num(0)); RECORDS];
    let mut alarms = [Alarms(num(0)); RECORDS];
    for i in 0 .. RECORDS {
        pos[i] = Pos(num(model.pos[i]));
        vel[i] = Vel(num(model.vel[i]));
        frozen[i] = Heat(num(model.heat[i]));
        edge[i] = Edge {
            west: Bool(i % WIDE == 0),
            east: Bool(i % WIDE == WIDE - 1),
        };
    }

    let world = (
        Lent::<Column<Pos>>::over(&mut pos),
        (
            Lent::<Column<Vel>>::over(&mut vel),
            (
                Lent::<Buffered<Heat, Exchange>>::over(&mut frozen, &mut live),
                (
                    Lent::<Column<Edge>>::over(&mut edge),
                    (
                        Lent::<Accum<Crossed>>::over(&mut crossed),
                        (
                            Lent::<Virtual<Overheat>>::unfired(),
                            (Lent::<Column<Alarms>>::over(&mut alarms), ()),
                        ),
                    ),
                ),
            ),
        ),
    );
    let mut oracle = Oracle::new(world, USize(RECORDS));
    let units = (Drift, (Spread, (Cross, (Alarm, (Tally, ())))));

    let mut frames_fired = 0;
    let mut values_crossed = 0;
    for frame in 0 .. FRAMES {
        oracle.frame(&units);
        model.frame();

        let got: Vec<u64> = oracle
            .of::<Column<Pos>, _>()
            .values()
            .iter()
            .map(|p| raw(p.0))
            .collect();
        assert_eq!(got, model.pos, "pos after frame {frame}");

        let got: Vec<u64> = oracle
            .of::<Buffered<Heat, Exchange>, _>()
            .frozen()
            .iter()
            .map(|h| raw(h.0))
            .collect();
        assert_eq!(got, model.heat, "heat after frame {frame}");

        let got: Vec<u64> = oracle
            .of::<Accum<Crossed>, _>()
            .appended()
            .iter()
            .map(|c| raw(c.0))
            .collect();
        assert_eq!(got, model.crossed, "crossed in frame {frame}");

        let got = oracle.of::<Virtual<Overheat>, _>().fired().0;
        assert_eq!(got, model.fired, "overheat in frame {frame}");

        let got: Vec<u64> = oracle
            .of::<Column<Alarms>, _>()
            .values()
            .iter()
            .map(|a| raw(a.0))
            .collect();
        assert_eq!(got, model.alarms, "alarms after frame {frame}");

        frames_fired += usize::from(model.fired);
        values_crossed += model.crossed.len();
    }

    // The run has to exercise what it claims to: the triggered unit both ran
    // and rested, and the accumulator took values.
    assert!(
        frames_fired > 0 && frames_fired < FRAMES,
        "overheat fired in {frames_fired} frames"
    );
    assert!(values_crossed > 0, "no record crossed");
}

/// FNV-1a over 64 bits, written out, to hold the fold against.
fn fnv1a(bytes: &[u8]) -> u64 {
    let mut state: u64 = 0xCBF2_9CE4_8422_2325;
    for byte in bytes {
        state ^= u64::from(*byte);
        state = state.wrapping_mul(0x100_0000_01B3);
    }
    state
}

#[test]
fn the_fold_is_fnv1a_over_the_words_least_significant_byte_first() {
    assert_eq!(Fold::new().finish().to_raw(), 0xCBF2_9CE4_8422_2325);

    let words: [u64; 3] = [1, 0x0102_0304_0506_0708, u64::MAX];
    let mut fold = Fold::new();
    let mut bytes = Vec::new();
    for word in words {
        fold.word(Bits::<64, Hot>::from_raw(word));
        bytes.extend_from_slice(&word.to_le_bytes());
    }
    assert_eq!(fold.finish().to_raw(), fnv1a(&bytes));

    let mut other = Fold::new();
    for word in [1, 0x0102_0304_0506_0709, u64::MAX] {
        other.word(Bits::<64, Hot>::from_raw(word));
    }
    assert_ne!(
        other.finish().to_raw(),
        fnv1a(&bytes),
        "one bit of one word changes the digest"
    );
}
