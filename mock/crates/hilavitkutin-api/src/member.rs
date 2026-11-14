//! Members of an access set, and the window a read with a reach is handed.
//!
//! A read or write list is a list of members. A store marker is a member
//! that stands for itself. `Reach`, `Written` and `Differs` are members that
//! stand for the store they wrap and say one more thing about how the unit
//! reads or writes it. A wrapper is never registered: it names a registered
//! store in a list.

use core::marker::PhantomData;

use arvo::{Cap, USize};
use arvo_tensor::cap_size;
use notko::Maybe;

use crate::accum_overflow::AccumOverflow;
use crate::builder_input::ExtensionSurface;
use crate::column_value::ColumnValue;
use crate::run_cfg::DefaultRunCfg;
use crate::store::{Accum, BufferMode, Buffered, Column, Resource, Virtual};

/// One cell of an access set.
///
/// `Store` is the store the member stands for. The plan's access masks, the
/// totals of `WorkUnitBundle` and `HoldsAll` read it, so a wrapper asks of
/// registration and of the schedule what the bare store would.
#[diagnostic::on_unimplemented(
    message = "`{Self}` is not a member of an access set",
    note = "A member is a store marker (`Resource<T>`, `Column<T>`, `Buffered<T, M>`, `Accum<T>`, `Virtual<T>`) or a wrapper over a `Column` or a `Buffered` (`Reach<C, N>`, `Written<C>`, `Differs<C>`). `Differs` asks `PartialEq` of the value type."
)]
pub trait Member: 'static {
    /// The store this member stands for.
    type Store;
}

impl<T: 'static> Member for Resource<T> {
    type Store = Self;
}

impl<T: 'static> Member for Column<T> {
    type Store = Self;
}

impl<T: 'static, M: BufferMode> Member for Buffered<T, M> {
    type Store = Self;
}

impl<T: 'static> Member for Accum<T> {
    type Store = Self;
}

impl<T: 'static> Member for Virtual<T> {
    type Store = Self;
}

impl<T: ?Sized + 'static> Member for ExtensionSurface<T> {
    type Store = Self;
}

// The run configuration is registered into the store list, where it stands
// for itself like every other registered input.
impl<O: AccumOverflow + 'static> Member for DefaultRunCfg<O> {
    type Store = Self;
}

macro_rules! wrapper_marker {
    ($(#[$doc:meta])* $name:ident < C $(, const $n:ident : $nt:ty)? >) => {
        $(#[$doc])*
        #[repr(transparent)]
        pub struct $name<C $(, const $n: $nt)?>(PhantomData<C>);

        impl<C $(, const $n: $nt)?> Copy for $name<C $(, $n)?> {}
        impl<C $(, const $n: $nt)?> Clone for $name<C $(, $n)?> {
            #[inline(always)]
            fn clone(&self) -> Self {
                *self
            }
        }
        impl<C $(, const $n: $nt)?> Default for $name<C $(, $n)?> {
            #[inline(always)]
            fn default() -> Self {
                $name(PhantomData)
            }
        }
        impl<C $(, const $n: $nt)?> $name<C $(, $n)?> {
            /// Construct the marker, to name the member as a value.
            pub const fn new() -> Self {
                $name(PhantomData)
            }
        }
    };
}

wrapper_marker! {
    /// Read `C` at the record and at up to `N` records to either side.
    ///
    /// A member of a read list. `C` is a `Column` or a `Buffered`, and the
    /// value a function reads of it is a [`Window`].
    Reach<C, const N: Cap>
}

wrapper_marker! {
    /// Write `C`, counted as changed wherever the unit wrote it.
    ///
    /// A member of a write list. `C` is a `Column` or a `Buffered`. No
    /// compare is made and nothing is asked of the value type.
    Written<C>
}

wrapper_marker! {
    /// Write `C`, counted as changed only where the value differs.
    ///
    /// A member of a write list. `C` is a `Column` or a `Buffered`. The value
    /// written is compared with the value the record held, so the value type
    /// is `PartialEq`. For a buffered column the value held is the frozen one.
    Differs<C>
}

impl<T: 'static, const N: Cap> Member for Reach<Column<T>, N> {
    type Store = Column<T>;
}

impl<T: 'static, M: BufferMode, const N: Cap> Member for Reach<Buffered<T, M>, N> {
    type Store = Buffered<T, M>;
}

impl<T: 'static> Member for Written<Column<T>> {
    type Store = Column<T>;
}

impl<T: 'static, M: BufferMode> Member for Written<Buffered<T, M>> {
    type Store = Buffered<T, M>;
}

impl<T: PartialEq + 'static> Member for Differs<Column<T>> {
    type Store = Column<T>;
}

impl<T: PartialEq + 'static, M: BufferMode> Member for Differs<Buffered<T, M>> {
    type Store = Buffered<T, M>;
}

/// A record of a column and the records to either side of it.
///
/// What a value function reads of a `Reach<C, N>` member. `here` is the
/// record the function is applied to, `before` and `after` are the records
/// `K` places earlier and later in the column's one sequence, and a `K`
/// greater than `N` does not compile. Nothing is read past either end of the
/// column: a neighbour that would be is `Maybe::Isnt`.
///
/// The refusal of a `K` past the reach is a monomorphisation error, so
/// `cargo check` does not report it and `cargo build` does. A window of
/// reach one read two records back is refused:
///
/// ```compile_fail,E0080
/// use arvo::USize;
/// use arvo_tensor::cap;
/// use hilavitkutin_api::Window;
///
/// let column = [1u32, 2, 3];
/// // SAFETY: three initialised records, and record 2 is one of them.
/// let w = unsafe { Window::<u32, { cap(1) }>::over(column.as_ptr(), USize(2), USize(3)) };
/// let _ = w.before::<{ cap(2) }>();
/// ```
///
/// and so is one read two records on:
///
/// ```compile_fail,E0080
/// use arvo::USize;
/// use arvo_tensor::cap;
/// use hilavitkutin_api::Window;
///
/// let column = [1u32, 2, 3];
/// // SAFETY: three initialised records, and record 0 is one of them.
/// let w = unsafe { Window::<u32, { cap(1) }>::over(column.as_ptr(), USize(0), USize(3)) };
/// let _ = w.after::<{ cap(2) }>();
/// ```
///
/// The same reads at the reach build:
///
/// ```
/// use arvo::USize;
/// use arvo_tensor::cap;
/// use hilavitkutin_api::Window;
///
/// let column = [1u32, 2, 3];
/// // SAFETY: three initialised records, and record 1 is one of them.
/// let w = unsafe {
///     Window::<u32, { cap(1) }>::over(column.as_ptr(), USize(1), USize(3))
/// };
/// let _ = (w.before::<{ cap(1) }>(), w.after::<{ cap(1) }>());
/// ```
pub struct Window<'a, T: ColumnValue, const N: Cap> {
    base:  *const T,
    at:    USize,
    count: USize,
    _side: PhantomData<&'a [T]>,
}

impl<'a, T: ColumnValue, const N: Cap> Copy for Window<'a, T, N> {}
impl<'a, T: ColumnValue, const N: Cap> Clone for Window<'a, T, N> {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, T: ColumnValue, const N: Cap> Window<'a, T, N> {
    /// A window on record `at` of the column that starts at `base` and
    /// holds `count` records. For a runner, which builds one per record.
    ///
    /// # Safety
    ///
    /// `base` points at `count` initialised records of `T` that stay valid
    /// and unwritten for `'a`, and `at` is below `count`.
    #[inline(always)]
    pub const unsafe fn over(base: *const T, at: USize, count: USize) -> Self {
        Window {
            base,
            at,
            count,
            _side: PhantomData,
        }
    }

    /// The value at the record the function is applied to.
    #[inline(always)]
    pub const fn here(&self) -> T {
        // SAFETY: `over` holds `at < count` over `count` valid records.
        unsafe { *self.base.add(self.at.0) }
    }

    /// The value `K` records earlier, or nothing before the first record.
    #[inline(always)]
    pub const fn before<const K: Cap>(&self) -> Maybe<T> {
        const {
            assert!(
                cap_size(K) <= cap_size(N),
                "a `Window` reads at most its reach `N` records before the record"
            )
        };
        let k = cap_size(K);
        if self.at.0 >= k {
            // lint:allow(no-bare-numeric) reason: record index compare against the const reach; tracked: #121
            // SAFETY: `at - k` is a record index, since `k <= at < count`.
            Maybe::Is(unsafe { *self.base.add(self.at.0 - k) })
        } else {
            Maybe::Isnt
        }
    }

    /// The value `K` records later, or nothing past the last record.
    #[inline(always)]
    pub const fn after<const K: Cap>(&self) -> Maybe<T> {
        const {
            assert!(
                cap_size(K) <= cap_size(N),
                "a `Window` reads at most its reach `N` records after the record"
            )
        };
        let k = cap_size(K);
        if self.count.0 - self.at.0 > k {
            // lint:allow(no-bare-numeric) reason: record index compare against the const reach; tracked: #121
            // SAFETY: `at + k < count`, so it is a record index.
            Maybe::Is(unsafe { *self.base.add(self.at.0 + k) })
        } else {
            Maybe::Isnt
        }
    }
}
