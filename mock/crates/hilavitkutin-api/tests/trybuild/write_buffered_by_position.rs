//! The positional `write` reaches a `Column`. A buffered column is written
//! through the value function alone, and the refusal says so.

use arvo::USize;
use hilavitkutin_api::{AccessSet, Buffered, ColumnWriterApi, ResolveColumnWrite, write};

#[derive(Clone, Copy, PartialEq)]
struct Temp(u32);

struct Only;
struct Ctx;

impl ResolveColumnWrite<Temp, Only> for Ctx {
    unsafe fn resolve_write(&self, _i: USize, _v: Temp) {}

    unsafe fn resolve_held(&self, _i: USize) -> Temp {
        Temp(0)
    }
}

impl<W: AccessSet> ColumnWriterApi<W> for Ctx {}

fn main() {
    let ctx = Ctx;
    unsafe {
        ColumnWriterApi::<write![Buffered<Temp>]>::write(&ctx, USize(0), Temp(1));
    }
}
