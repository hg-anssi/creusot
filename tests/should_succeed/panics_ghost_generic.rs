extern crate creusot_std;
use creusot_std::{ghost::FnGhost, prelude::*};

#[requires(forall <x> f.precondition((x)))]
#[requires(forall <x> !f.panic_condition((x)))]
pub fn checked_id<F: Fn(u32) -> u32 + FnGhost>(f: F) -> u32 {
    ghost! {
        let _ = f(32);
    };
    f(50)
}
