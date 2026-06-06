//! Sketch (E8 / #340, Phase E): adapt observe/tune with hysteresis (R5).
//!
//! Each AdaptAxis folds an EMA `(ema*7 + measured) >> 3` (gated by sample_skip),
//! compares thresholds, and triggers morsel re-size / per-phase config re-select /
//! full re-plan, all at ScheduleEnd between frames. The danger is OSCILLATION: a
//! metric that hovers near the threshold flips the decision every frame, causing
//! the engine to thrash (re-size, re-plan, re-size, ...). E8 (roadmap section 9):
//! hysteresis (dead-band + K-consecutive crossing + per-trigger cooldown) prevents
//! the oscillation.
//!
//! Hypothesis: under a synthetic metric stream that hovers around the threshold
//! with noise, a NAIVE comparator (flip whenever ema crosses the single threshold)
//! transitions many times (oscillates), while the HYSTERESIS comparator (separate
//! high/low thresholds = dead-band, require K consecutive readings past the band,
//! then a cooldown before the next flip) transitions only a handful of times,
//! tracking the genuine trend and ignoring the noise. Expect hysteresis
//! transitions to be a small fraction of naive transitions. Leeway (section 9):
//! SOME-SHAPE. Outcome at the bottom.

#![allow(dead_code)]

use arvo::USize;

// EMA fold, the canonical (ema*7 + measured) >> 3 (alpha = 1/8). Fixed-point on
// u32 (the engine uses the arvo fixed-point EMA; the fold shape is the point).
#[inline]
fn ema_step(ema: u32, measured: u32) -> u32 {
    (ema.wrapping_mul(7).wrapping_add(measured)) >> 3
}

// Naive comparator: decision = ema >= threshold; flips state every crossing.
struct Naive {
    threshold: u32,
    state: bool,
    transitions: u32,
}
impl Naive {
    fn observe(&mut self, ema: u32) {
        let want = ema >= self.threshold;
        if want != self.state {
            self.state = want;
            self.transitions += 1;
        }
    }
}

// Hysteresis comparator: a dead-band [lo, hi] around the threshold, K consecutive
// readings past the relevant edge required to flip, and a cooldown after a flip
// during which no further flip is allowed. This is the R5 anti-oscillation triad.
struct Hysteresis {
    lo: u32,
    hi: u32,
    k: u32,
    cooldown: u32,
    state: bool,
    run: u32,        // consecutive readings past the active edge
    cool: u32,       // frames remaining in cooldown
    transitions: u32,
}
impl Hysteresis {
    fn observe(&mut self, ema: u32) {
        if self.cool > 0 {
            self.cool -= 1;
            self.run = 0;
            return;
        }
        // To flip ON, ema must exceed the HIGH edge for k consecutive frames; to
        // flip OFF, drop below the LOW edge for k consecutive. Inside the band the
        // run resets (no pressure to flip).
        let past_edge = if self.state { ema < self.lo } else { ema >= self.hi };
        if past_edge {
            self.run += 1;
            if self.run >= self.k {
                self.state = !self.state;
                self.transitions += 1;
                self.run = 0;
                self.cool = self.cooldown;
            }
        } else {
            self.run = 0;
        }
    }
}

// Deterministic synthetic metric stream: a slow genuine trend (ramp up then down)
// plus high-frequency noise that straddles the threshold. No RNG (banned + breaks
// reproducibility); noise is a fixed oscillation pattern.
fn metric(frame: u32, threshold: u32) -> u32 {
    // Genuine trend: triangle wave over 400 frames, amplitude ~+-40 around thresh.
    let t = frame % 400;
    let trend = if t < 200 { t } else { 400 - t }; // 0..200..0
    let trend = threshold as i64 - 40 + (trend as i64 * 80 / 200); // thresh-40 .. thresh+40
    // Noise: +-15 sawtooth straddling the threshold every other frame.
    let noise = if frame % 2 == 0 { 15 } else { -15 };
    (trend + noise).max(0) as u32
}

fn main() {
    let threshold = 1000u32;
    let frames = 4000u32;
    let _ = USize(frames as usize);

    let mut naive = Naive { threshold, state: false, transitions: 0 };
    // Dead-band +-25 around threshold; require 4 consecutive; 16-frame cooldown.
    let mut hyst = Hysteresis {
        lo: threshold - 25,
        hi: threshold + 25,
        k: 4,
        cooldown: 16,
        state: false,
        run: 0,
        cool: 0,
        transitions: 0,
    };

    let mut ema = threshold; // seed at threshold (worst case for oscillation)
    for frame in 0..frames {
        let measured = metric(frame, threshold);
        ema = ema_step(ema, measured);
        naive.observe(ema);
        hyst.observe(ema);
    }

    println!(
        "over {frames} frames near threshold {threshold}: naive transitions = {}, hysteresis \
         transitions = {} (dead-band +-25, k=4, cooldown=16)",
        naive.transitions, hyst.transitions
    );
    // The EMA already smooths a lot; hysteresis must still cut transitions to a
    // small fraction of naive AND track the genuine trend (a few flips, not zero).
    assert!(
        hyst.transitions * 3 < naive.transitions.max(1),
        "hysteresis must cut oscillation to <1/3 of naive (naive={}, hyst={})",
        naive.transitions,
        hyst.transitions
    );
    assert!(
        hyst.transitions <= 40,
        "hysteresis tracks the slow trend with few flips, not per-frame thrash (got {})",
        hyst.transitions
    );
    println!(
        "WORKS: adapt EMA + hysteresis prevents oscillation. The (ema*7+measured)>>3 fold plus a \
         dead-band + K-consecutive + cooldown triad cut decision transitions to well under a \
         third of the naive single-threshold comparator under a threshold-straddling metric \
         stream, while still tracking the genuine trend. Adapt fires between frames at \
         ScheduleEnd without thrashing morsel-resize / re-plan."
    );
}

// ---------------------------------------------------------------------
// OUTCOME: WORKS (nightly-2026-05-28).
//
// Over 4000 frames of a threshold-straddling metric stream (slow triangle trend +
// per-frame +-15 noise around threshold 1000), the naive single-threshold
// comparator made 100 decision transitions (oscillation) while the hysteresis
// comparator (dead-band +-25, k=4 consecutive, 16-frame cooldown) made 20 = 1/5
// the transitions, tracking the genuine up/down trend without per-frame thrash.
//
// WHAT THIS SETTLES (E8): the adapt EMA fold (ema*7+measured)>>3 plus the R5
// anti-oscillation triad (dead-band + K-consecutive crossing + per-trigger
// cooldown) prevents the morsel-resize / re-plan thrash a naive threshold compare
// would cause near a boundary. Adapt fires between frames at ScheduleEnd (E4 meta
// loop) and the decision is stable. The fold + hysteresis are pure integer ops
// (the engine uses arvo fixed-point; same shape).
//
// WHAT THIS DOES NOT SETTLE: the exact per-axis thresholds / k / cooldown (tuned
// per AdaptAxis against real metric distributions, a bench/tuning task) and the
// re-plan rate-limit specifics; this proves the anti-oscillation MECHANISM, which
// is the load-bearing correctness property (stability), not the tuned constants.
// ---------------------------------------------------------------------
