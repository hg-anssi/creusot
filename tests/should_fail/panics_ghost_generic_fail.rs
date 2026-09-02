// WHY3PROVE ENABLE_PANICS_ONLY
extern crate creusot_std;
use creusot_std::{ghost::FnGhost, prelude::*};

#[requires(forall <x> f.precondition((x,)))]
#[may_panic(true)]
pub fn checked_id<F: Fn(u32) -> u32 + FnGhost>(f: F) {
    ghost! {
        // even if global fonction is allowed to panic, this block is not!
        let _ = f(32);
    };
}
