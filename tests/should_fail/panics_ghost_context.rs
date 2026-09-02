// WHY3PROVE ENABLE_PANICS_ONLY
//! A *named* ghost function may carry a panic clause and is then **dual-use**:
//!  - in a program context it may actually panic;
//!  - in a ghost context, `panics(P)` behaves exactly like `requires(!P)` — the
//!    call site must prove `!P` — because a ghost caller has no `panic`
//!    continuation, so the call threads the "the callee cannot panic here"
//!    obligation.
//!
//! This is the mechanism that lets `creusot-std` expose a single
//! `#[check(ghost)] #[panics(P)]` accessor (e.g. `Option::unwrap`) instead of a
//! panicking program version plus a duplicated total `_ghost` variant. Such
//! declarations are always *trusted* (extern_spec / `#[trusted(ghost)]`): a
//! non-trusted ghost body could not contain the panicking operation in the first
//! place (its purity context forbids calling an impure/panicking function).

extern crate creusot_std;
use creusot_std::prelude::*;

/// The dual-use accessor: a named, ghost-callable function that may panic on
/// `x == 0`. Trusted, standing in for an `extern_spec!` declaration.
#[check(ghost)]
#[panics(x@ == 0)]
#[ensures(result@ == x@)]
pub fn checked_id(x: u32) -> u32 {
    if x == 0 { panic!("zero") } else { x }
}

/// Ghost context: inside a `ghost!` block the call is well-formed only if `!P`
/// (here `n != 0`) is provable — this is the `may_panic(P) ≡ requires(!P)` fold.
// #[requires(n@ != 0)]
#[may_panic(n@ == 0)]
pub fn ghost_use(n: u32) {
    ghost! {
        let _ = checked_id(n);
    };
}
