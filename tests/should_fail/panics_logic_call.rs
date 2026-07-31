extern crate creusot_std;
use creusot_std::prelude::*;

#[may_panic(true)]
fn may_panic_fn() -> bool {
    true
}

// A logic body may only call logic functions; a `#[may_panic]` function is a
// program function, so calling it from a logic context is rejected. This is a
// Creusot-level (translation-time) check.
#[logic]
pub fn calls_from_logic() -> bool {
    may_panic_fn()
}
