//! A `#[check(ghost)]` closure MAY now carry `#[may_panic(P)]` (it used to be
//! rejected outright).
//!
//! Such a closure is wrapped in `FnGhostWrapper`, which is a *transparent forwarder*:
//! its `precondition`, `postcondition` and `panic_condition` all delegate to the inner
//! closure, and `FnGhostWrapper::call` propagates the panic via
//! `#[may_panic(self.panic_condition(args))]`. So a ghost closure is **dual-use**,
//! exactly like a named ghost function: its panic *propagates* in a `may_panic` program
//! context, and folds to `requires(!P)` in a total context or a `ghost!` block — all
//! through the standard `may_panic` machinery (no `FnGhost ⟹ panic_condition ≡ false`
//! law, no bespoke precondition strengthening). A non-panicking ghost closure still has
//! `panic_condition ≡ false`, by delegation.
//!
//! NB: the body genuinely panics — panic entry points are themselves
//! `#[check(ghost)] #[requires(false)]` (creusot-std `std/panicking.rs`), hence
//! ghost-callable, so `panic!` is allowed in a ghost body. Because the closure
//! `may_panic(y == 0)`, that `panic!` is routed to the closure's `panic` exit
//! (proving the condition `y == 0`) rather than treated as unreachable.
//!
//! Contrast: tests/should_fail/panics_logic.rs (logic functions are still rejected)
//! and tests/should_succeed/panics_ghost_dual.rs (named ghost functions).

extern crate creusot_std;
use creusot_std::{ghost::FnGhost, prelude::*};

/// A `FnGhost`-bounded sink that *calls* `f` and *propagates* its panic: a named ghost
/// function that forwards the closure's `may_panic` condition. Generic over `F: FnGhost`,
/// so `f.panic_condition` stays opaque here and is resolved at each concrete call site.
#[requires(f.precondition((n,)))]
#[may_panic(f.panic_condition((n,)))]
#[check(ghost)]
pub fn apply<F: Fn(u32) -> u32 + FnGhost>(f: F, n: u32) -> u32 {
    f(n)
}

/// Caller: builds a ghost closure that may panic at `0`, then discharges `apply`'s
/// strengthened precondition. Because the wrapper's precondition now carries `!P`
/// (`n != 0`), this requires `n@ != 0` — which the caller obtains from its own
/// `requires`. Drop that `requires` and the call no longer verifies.
#[requires(n@ != 0)]
pub fn use_requires(n: u32) -> u32 {
    // f is of type FnGhostWrapper
    let f = #[check(ghost)]
    #[may_panic(y@ == 0)]
    |y: u32| -> u32 { if y == 0 { panic!() } else { y } };
    let _ = f(n);
    apply(f, n)
}

pub fn define_ghost_maypanic_in_empty(_n: u32) {
    let _ = #[check(ghost)]
    #[may_panic(y@ == 0)]
    |y: u32| -> u32 { if y == 0 { panic!() } else { y } };
}

#[requires(n@ != 0)]
pub fn use_ghost_maypanic_in_requires(n: u32) -> u32 {
    // f is of type FnGhostWrapper
    let f = #[check(ghost)]
    #[may_panic(y@ == 0)]
    |y: u32| -> u32 { if y == 0 { panic!() } else { y } };
    f(n)
}

#[may_panic(n@ == 0)]
pub fn use_ghost_maypanic_in_maypanic(n: u32) -> u32 {
    // f is of type FnGhostWrapper
    let f = #[check(ghost)]
    #[may_panic(y@ == 0)]
    |y: u32| -> u32 { if y == 0 { panic!() } else { y } };
    f(n)
}

#[requires(n@ != 0)]
pub fn use_closure_in_ghost_requires(n: u32) {
    // f is of type FnGhostWrapper
    let f = #[check(ghost)]
    #[may_panic(y@ == 0)]
    |y: u32| -> u32 { if y == 0 { panic!() } else { y } };
    ghost! {
        let _ = f(n);
    };
}

pub fn use_closure_in_ghost_value(n: u32) {
    // f is of type FnGhostWrapper
    let f = #[check(ghost)]
    #[may_panic(y@ == 0)]
    |y: u32| -> u32 { if y == 0 { panic!() } else { y } };
    ghost! {
        let _ = f(54);
    };
}
