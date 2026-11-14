//! The frame of the nine units, spelled out by hand over plain arrays.
//!
//! No work unit, no access set and no store is named here. The plane is
//! walked by its two coordinates, and each step of a frame is written as what
//! it does to the arrays, in the sequence the units are registered.

use arvo::{Bool, USize};
use arvo_tensor::cap_size;

use crate::values::{
    Alarms,
    Ash,
    Crossed,
    Edge,
    Glow,
    Heat,
    LIMIT,
    Level,
    Num,
    Peak,
    Pos,
    RECORDS,
    TALL,
    THRESHOLD,
    Vel,
    WIDE,
    ZERO,
    higher,
};

/// Every third record moves at the same speed.
const SPEEDS: USize = USize(3);
/// Every place this many records apart starts warm.
const WARM_EVERY: USize = USize(11);
/// The heat a warm place starts with.
const WARMTH: Num = Num::from_raw(7);
/// The level every record but the dip starts at.
const FULL: Num = Num::from_raw(9);
/// The level the dip starts at.
const LOW: Num = Num::from_raw(3);
/// The first record of the dip.
const DIP_FROM: USize = USize(10);
/// The record after the last of the dip.
const DIP_TO: USize = USize(13);

/// The sum of two numbers.
const fn sum(one: Num, other: Num) -> Num {
    Num::from_raw(one.to_raw() + other.to_raw())
}

/// The number a count is.
const fn num(count: USize) -> Num {
    Num::from_raw(count.0 as _)
}

/// The state of one run between two frames.
#[derive(Copy, Clone)]
pub struct Model {
    /// The positions.
    pub pos:      [Pos; RECORDS.0],
    /// The velocities.
    pub vel:      [Vel; RECORDS.0],
    /// The heat every read of the next frame sees.
    pub heat:     [Heat; RECORDS.0],
    /// The sides of the plane each place lies on.
    pub edge:     [Edge; RECORDS.0],
    /// The alarm counts.
    pub alarms:   [Alarms; RECORDS.0],
    /// The levels every read of the next frame sees.
    pub level:    [Level; RECORDS.0],
    /// The peaks.
    pub peak:     [Peak; RECORDS.0],
    /// The glow.
    pub glow:     [Glow; RECORDS.0],
    /// The ash.
    pub ash:      [Ash; RECORDS.0],
    /// Whether the last frame fired `Overheat`.
    pub overheat: Bool,
    /// What the last frame appended, in record sequence, in the first
    /// `crossed` places.
    pub appended: [Crossed; RECORDS.0],
    /// How many values the last frame appended.
    pub crossed:  USize,
}

impl Model {
    /// The state every run begins from.
    pub const fn new() -> Self {
        let wide = cap_size(WIDE);
        let mut model = Model {
            pos:      [Pos(ZERO); RECORDS.0],
            vel:      [Vel(ZERO); RECORDS.0],
            heat:     [Heat(ZERO); RECORDS.0],
            edge:     [Edge {
                west: Bool(false),
                east: Bool(false),
            }; RECORDS.0],
            alarms:   [Alarms(ZERO); RECORDS.0],
            level:    [Level(FULL); RECORDS.0],
            peak:     [Peak(ZERO); RECORDS.0],
            glow:     [Glow(ZERO); RECORDS.0],
            ash:      [Ash(ZERO); RECORDS.0],
            overheat: Bool(false),
            appended: [Crossed(ZERO); RECORDS.0],
            crossed:  USize(0),
        };
        let mut y = 0;
        while y < TALL.0 {
            let mut x = 0;
            while x < wide {
                let at = y * wide + x;
                model.pos[at] = Pos(num(USize(at)));
                model.vel[at] = Vel(num(USize(1 + at % SPEEDS.0)));
                if at.is_multiple_of(WARM_EVERY.0) {
                    model.heat[at] = Heat(WARMTH);
                }
                model.edge[at] = Edge {
                    west: Bool(x == 0),
                    east: Bool(x + 1 == wide),
                };
                if at >= DIP_FROM.0 && at < DIP_TO.0 {
                    model.level[at] = Level(LOW);
                }
                x += 1;
            }
            y += 1;
        }
        model
    }

    /// One frame: what the nine units leave behind, and the boundary.
    pub const fn frame(&mut self) {
        let wide = cap_size(WIDE);
        let began = *self;

        // the positions move
        let mut at = 0;
        while at < RECORDS.0 {
            self.pos[at] = Pos(sum(began.pos[at].0, began.vel[at].0));
            at += 1;
        }

        // the heat a place began with, plus that of the places beside it on
        // the plane
        let mut y = 0;
        while y < TALL.0 {
            let mut x = 0;
            while x < wide {
                let at = y * wide + x;
                let mut heat = began.heat[at].0;
                if x > 0 {
                    heat = sum(heat, began.heat[at - 1].0);
                }
                if x + 1 < wide {
                    heat = sum(heat, began.heat[at + 1].0);
                }
                if y > 0 {
                    heat = sum(heat, began.heat[at - wide].0);
                }
                if y + 1 < TALL.0 {
                    heat = sum(heat, began.heat[at + wide].0);
                }
                self.heat[at] = Heat(heat);
                x += 1;
            }
            y += 1;
        }

        // a position that was under the threshold as the frame began and is
        // not any more
        self.crossed = USize(0);
        let mut at = 0;
        while at < RECORDS.0 {
            let from = began.pos[at].0.to_raw();
            let to = self.pos[at].0.to_raw();
            if from < THRESHOLD.to_raw() && to >= THRESHOLD.to_raw() {
                self.appended[self.crossed.0] = Crossed(self.pos[at].0);
                self.crossed = USize(self.crossed.0 + 1);
            }
            at += 1;
        }

        // overheated, where a place began the frame at or over the limit
        self.overheat = Bool(false);
        let mut at = 0;
        while at < RECORDS.0 {
            if began.heat[at].0.to_raw() >= LIMIT.to_raw() {
                self.overheat = Bool(true);
            }
            at += 1;
        }

        // every count goes up in an overheated frame
        if self.overheat.0 {
            let mut at = 0;
            while at < RECORDS.0 {
                self.alarms[at] = Alarms(sum(began.alarms[at].0, Num::from_raw(1)));
                at += 1;
            }
        }

        // the level of a record and of the records before and after it, as
        // the frame began
        let mut at = 0;
        while at < RECORDS.0 {
            let mut level = began.level[at].0;
            if at > 0 {
                level = higher(level, began.level[at - 1].0);
            }
            if at + 1 < RECORDS.0 {
                level = higher(level, began.level[at + 1].0);
            }
            self.level[at] = Level(level);
            at += 1;
        }

        // the peak, the glow and the ash
        let mut at = 0;
        while at < RECORDS.0 {
            self.peak[at] = Peak(higher(began.peak[at].0, began.level[at].0));
            let glow = sum(self.pos[at].0, self.pos[at].0);
            self.glow[at] = Glow(glow);
            self.ash[at] = Ash(sum(glow, LIMIT));
            at += 1;
        }
    }
}

impl Default for Model {
    fn default() -> Self {
        Self::new()
    }
}
