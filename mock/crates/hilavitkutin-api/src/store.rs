//! Store marker types.
//!
//! Zero-sized type-level descriptors for the three store shapes
//! (`Resource`, `Column`, `Virtual`) and the three resource layouts
//! (`Field`, `Seq`, `Map`). The scheduler sees `StoreId` pairs at
//! runtime; these types are the compile-time identity erased to
//! plain indices by execution time.

use core::marker::PhantomData;

use arvo::newtype::Cap;

use crate::column_value::ColumnValue;

/// Singleton store: one value shared across the pipeline.
#[repr(transparent)]
pub struct Resource<T>(PhantomData<T>);

impl<T> Copy for Resource<T> {}
impl<T> Clone for Resource<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> Default for Resource<T> {
    #[inline(always)]
    fn default() -> Self {
        Resource(PhantomData)
    }
}

/// Collection store: N records per column, morsel-chunked.
#[repr(transparent)]
pub struct Column<T>(PhantomData<T>);

impl<T> Copy for Column<T> {}
impl<T> Clone for Column<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> Default for Column<T> {
    #[inline(always)]
    fn default() -> Self {
        Column(PhantomData)
    }
}

/// Zero-data store: DAG edge only. Used for fire flags.
#[repr(transparent)]
pub struct Virtual<T>(PhantomData<T>);

impl<T> Copy for Virtual<T> {}
impl<T> Clone for Virtual<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> Default for Virtual<T> {
    #[inline(always)]
    fn default() -> Self {
        Virtual(PhantomData)
    }
}

/// Scalar resource layout: ≤16-byte value, LLVM-promotable.
#[repr(transparent)]
pub struct Field<T: ColumnValue>(PhantomData<T>);

impl<T: ColumnValue> Copy for Field<T> {}
impl<T: ColumnValue> Clone for Field<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}
impl<T: ColumnValue> Default for Field<T> {
    #[inline(always)]
    fn default() -> Self {
        Field(PhantomData)
    }
}

/// Const-sized sequence resource layout. Arena-allocated.
#[repr(transparent)]
pub struct Seq<T, const N: Cap>(PhantomData<T>);

impl<T, const N: Cap> Copy for Seq<T, N> {}
impl<T, const N: Cap> Clone for Seq<T, N> {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}
impl<T, const N: Cap> Default for Seq<T, N> {
    #[inline(always)]
    fn default() -> Self {
        Seq(PhantomData)
    }
}

/// Const-sized key/value resource layout. Arena-allocated.
#[repr(transparent)]
pub struct Map<K, V, const N: Cap>(PhantomData<(K, V)>);

impl<K, V, const N: Cap> Copy for Map<K, V, N> {}
impl<K, V, const N: Cap> Clone for Map<K, V, N> {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}
impl<K, V, const N: Cap> Default for Map<K, V, N> {
    #[inline(always)]
    fn default() -> Self {
        Map(PhantomData)
    }
}
