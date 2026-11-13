//! The position mask at list lengths 8 and 32, at the default recursion limit.

mod common;

use common::*;

type Lo8 = list!(L0 L1 L2 L3);
type Hi8 = list!(L4 L5 L6 L7);
type R8 = list!(L0 L1 L2 L3 L4 L5 L6 L7);

type Lo32 = list!(
    L0 L1 L2 L3 L4 L5 L6 L7 L8 L9 L10 L11 L12 L13 L14 L15
);
type Hi32 = list!(
    L16 L17 L18 L19 L20 L21 L22 L23 L24 L25 L26 L27 L28 L29 L30 L31
);
type R32 = list!(
    L0 L1 L2 L3 L4 L5 L6 L7 L8 L9 L10 L11 L12 L13 L14 L15
    L16 L17 L18 L19 L20 L21 L22 L23 L24 L25 L26 L27 L28 L29 L30 L31
);

writes_in!(R8: R8, Lo8, Hi8);
writes_in!(R32: R32, Lo32, Hi32);

at_length!(the_mask_evaluates_at_length_8, R8, Lo8, Hi8, 8);
at_length!(the_mask_evaluates_at_length_32, R32, Lo32, Hi32, 32);
