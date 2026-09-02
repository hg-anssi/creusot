extern crate creusot_std;
use creusot_std::prelude::*;

// Logic functions have no operational panic outcome, so they cannot be
// specified to panic. This is a Creusot-level (translation-time) check, so it
// needs a file that is otherwise valid Rust.
#[logic]
#[may_panic(true)]
pub fn in_logic() -> bool {
    true
}
