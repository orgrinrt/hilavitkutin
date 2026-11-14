//! The two declarative kits the scheduler builder tests register, and the
//! stores they own.

use arvo::USize;
use hilavitkutin_api::footprint::ResourceFootprint;
use hilavitkutin_api::{BuilderInput, Column, Cons, Empty, Resource};
use hilavitkutin_kit::{Kit, KitDispatch};

// `Interner` is registered as a `Resource` value (`.with(Resource::new(
// Interner))`), so it must be `ColumnValue` (`Copy + 'static`).
// `Workspace` and `FileInfo` are only Kit-owned / column markers (never
// drained as resource values), so they need no derive.
#[derive(Copy, Clone)]
pub struct Interner;
pub struct Workspace;
#[derive(Copy, Clone, Default)]
pub struct FileInfo;

// Bare markers with no `Seq`/`Map` collection members, so their L1 morsel
// footprint is zero.
impl ResourceFootprint for Interner {
    const L1_BYTES: USize = USize(0);
}

impl ResourceFootprint for Workspace {
    const L1_BYTES: USize = USize(0);
}

pub struct InternerKit;

impl BuilderInput for InternerKit {
    type Dispatch = KitDispatch<Self>;
    type Init = Self;
}

impl Kit for InternerKit {
    type Owned = Cons<Resource<Interner>, Empty>;
    type Units = Empty;
}

pub struct WorkspaceKit;

impl BuilderInput for WorkspaceKit {
    type Dispatch = KitDispatch<Self>;
    type Init = Self;
}

impl Kit for WorkspaceKit {
    type Owned = Cons<Column<FileInfo>, Cons<Resource<Workspace>, Empty>>;
    type Units = Empty;
}
