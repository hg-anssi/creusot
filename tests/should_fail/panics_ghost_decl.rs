extern crate creusot_std;
use creusot_std::prelude::*;

// A ghost function is erased at runtime and must terminate successfully, so it
// cannot panic: specifying it with `#[may_panic]` is rejected by Creusot
// (`check_panics_allowed`), mirroring the existing rejection on logic functions.
// This also closes a breach: a ghost function is callable from a ghost context,
// so a `may_panic` ghost function would let a panic outcome leak into the erased
// world.
#[check(ghost)]
#[may_panic(true)]
pub fn ghost_may_panic() {}
