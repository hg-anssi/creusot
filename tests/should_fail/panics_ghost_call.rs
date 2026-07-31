extern crate creusot_std;
use creusot_std::prelude::*;

// A `#[may_panic]` function is necessarily a *program* function (purity < Ghost),
// so it can never be called from a `ghost!` block: ghost code is erased and has
// no operational panic outcome. This is a Creusot-level (translation-time) check.
#[may_panic(true)]
fn may_panic_fn() {}

pub fn calls_from_ghost() {
    ghost! {
        may_panic_fn();
    };
}
