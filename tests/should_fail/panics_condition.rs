// WHY3PROVE ENABLE_PANICS_ONLY

extern crate creusot_std;
use creusot_std::prelude::*;

/// A generic higher-order function that calls its closure argument through the
/// `Fn` bound and *propagates* the closure's panic condition to its own, via
/// `f.panic_condition(...)`.
#[requires(f.precondition((x,)))]
#[panics(f.panic_condition((x,)))]
#[ensures(f.postcondition((x,), result))]
pub fn apply_propagate<F: Fn(u32) -> u32>(f: F, x: u32) -> u32 {
    f(x)
}

#[may_panic(true)]
pub fn fake_panic(n: u32) -> u32 {
    n
}

#[panics(true)]
pub fn testlk() -> u32 {
    apply_propagate(fake_panic, 5)
}
