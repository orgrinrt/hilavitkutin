//! The positional `read` is bounded on the read list holding the bare
//! `Column<T>`. A buffered column is read through the value function alone.

use arvo::USize;
use hilavitkutin_api::{
    AccessSet,
    Buffered,
    Column,
    ColumnReaderApi,
    ColumnValue,
    Holds,
    ResolveColumnRead,
    read,
};

#[derive(Clone, Copy, PartialEq)]
struct Temp(u32);

struct Only;
struct Ctx;

impl ResolveColumnRead<Temp, Only> for Ctx {
    unsafe fn resolve_read(&self, _i: USize) -> Temp {
        Temp(0)
    }
}

impl<R: AccessSet> ColumnReaderApi<R> for Ctx {
    unsafe fn read<T: ColumnValue, I>(&self, i: USize) -> T
    where
        R: Holds<Column<T>>,
        Self: ResolveColumnRead<T, I>,
    {
        unsafe { self.resolve_read(i) }
    }
}

fn main() {
    let ctx = Ctx;
    let _: Temp = unsafe { ColumnReaderApi::<read![Buffered<Temp>]>::read(&ctx, USize(0)) };
}
