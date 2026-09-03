// Probe: can replace_resource install through Selector, and do the two
// index witnesses coincide?
//
// Not compiled as part of the workspace. Written to be pasted into
// scheduler/mod.rs during the round that implements D1.

// The shape under test:
//
// pub fn replace_resource<T: PlanAffecting, Index>(&mut self, new: T)
// where
//     Stores: Locate<T, Index>,
//     Index: WitnessIndex,
//     <Vals as BindingsFor>::Bindings: Selector<T, Index>,
// {
//     let p = Selector::<T, Index>::get(&self.bindings).as_ptr();
//     // SAFETY: the binding's PhantomData<T> witnesses the pointer was
//     // recorded for a T at drain; the scheduler holds &mut self so no
//     // worker is running and no reader aliases it.
//     unsafe { *p = new; }
//     self.mark_dirty::<T, Index>();
// }
