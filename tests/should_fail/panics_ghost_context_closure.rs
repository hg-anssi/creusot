// WHY3PROVE ENABLE_PANICS_ONLY
extern crate creusot_std;
use creusot_std::prelude::*;

#[may_panic(true)]
pub fn use_closure_in_ghost_requires(n: u32) {
    // f is of type FnGhostWrapper
    let f = #[check(ghost)]
    #[may_panic(y@ == 0)]
    |y: u32| -> u32 { if y == 0 { panic!() } else { y } };
    ghost! {
        let _ = f(n);
    };
}
