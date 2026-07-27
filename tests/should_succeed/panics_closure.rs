//! Closures may carry `#[may_panic(...)]` / `#[panics(...)]` clauses. This file
//! covers the *direct call* case (the closure is called where it is defined), for
//! which the panic threads through the closure's own contract.

extern crate creusot_std;
use creusot_std::prelude::*;

/// A closure that may panic, called directly by a caller that also may panic:
/// the closure's panic (on `y == 0`) propagates to the caller's panic outcome.
#[may_panic(x@ == 0)]
pub fn direct_propagate(x: u32) -> u32 {
    let c = #[may_panic(y@ == 0)]
    |y: u32| -> u32 { if y == 0 { panic!("zero") } else { y } };
    c(x)
}

/// A non-panicking caller calls a may-panic closure but discharges its panic
/// condition (here the argument is a non-zero constant). Since specifying the
/// closure disables spec inference, its return value is given explicitly.
#[ensures(result@ == 5)]
pub fn direct_discharge() -> u32 {
    let c = #[may_panic(y@ == 0)]
    #[ensures(result@ == y@)]
    |y: u32| -> u32 { if y == 0 { panic!("zero") } else { y } };
    c(5)
}

/// The `#[panics(P)]` sugar on a closure: panics iff `y == 0`.
#[panics(x@ == 0)]
pub fn direct_panics_sugar(x: u32) -> u32 {
    let c = #[panics(y@ == 0)]
    |y: u32| -> u32 { if y == 0 { panic!("zero") } else { y } };
    c(x)
}

/// A generic higher-order function that calls its closure argument through the
/// `Fn` bound and *propagates* the closure's panic condition to its own, via
/// `f.panic_condition(...)`.
#[requires(f.precondition((x,)))]
#[may_panic(f.panic_condition((x,)))]
#[ensures(f.postcondition((x,), result))]
pub fn apply_propagate<F: Fn(u32) -> u32>(f: F, x: u32) -> u32 {
    f(x)
}

/// A generic, non-panicking higher-order function: it *discharges* the closure's
/// panic condition by requiring it to be false here.
#[requires(f.precondition((x,)))]
#[requires(!f.panic_condition((x,)))]
#[ensures(f.postcondition((x,), result))]
pub fn apply_total<F: Fn(u32) -> u32>(f: F, x: u32) -> u32 {
    f(x)
}
