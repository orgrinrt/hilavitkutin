//! A build without `static` carries no memory provider over a buffer.

use hilavitkutin_providers::StaticBumpMemory;

fn main() {
    let _ = core::mem::size_of::<StaticBumpMemory>();
}
