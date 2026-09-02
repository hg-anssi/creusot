extern crate creusot_std;
use creusot_std::prelude::*;

/// Note: arithmetic overflow is still modeled as a hard proof obligation (a
/// precondition of the arithmetic operations), even in functions that may
/// panic. Here the panic case is made explicit instead.
#[panics(x == u32::MAX)]
#[ensures(result@ == x@ + 1)]
pub fn increment(x: u32) -> u32 {
    x + 1
}

/// The function returns `x / y`, or panics (only possible when `y == 0`).
#[may_panic(y@ == 0)]
#[ensures(result@ == x@ / y@)]
pub fn div(x: u32, y: u32) -> u32 {
    x / y
}

#[may_panic(x@ + y@ > u64::MAX@)]
pub fn add(x: u64, y: &u64) -> u64 {
    x + y
}
