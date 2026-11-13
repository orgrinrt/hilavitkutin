//! Two morsels opened at once, one `each` inside the other, and an index of
//! the outer one read through the inner block, which a shared borrow lets the
//! compiler shorten. The twin reads the inner block at an index of its own.

#![forbid(unsafe_code)]

use core::num::NonZeroUsize;

use engine::Morsels;

fn main() {
    let read = [1, 2, 3, 4];
    let mut write = [0; 4];
    let mut morsels = Morsels::new(&read, &mut write, NonZeroUsize::new(2).unwrap()).unwrap();
    let mut first = morsels.next().unwrap();
    let mut second = morsels.next().unwrap();
    first.each(|outer| {
        let kept = outer.indices().next().unwrap();
        second.each(|inner| {
            let mine = inner.indices().next().unwrap();
            let probe = mine;
            let _ = kept;
            let _ = inner.read(probe);
        });
    });
}
