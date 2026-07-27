//! Specifying functions that are allowed to panic.
//!
//! A function may declare a `#[may_panic(P)]` clause: it is then allowed to panic,
//! and only in (initial) states where `P` holds. `P` is a predicate over the
//! function inputs — `result` is not in scope, since it does not exist in the
//! panic outcome. Several `#[may_panic(...)]` clauses are *disjoined*: each one adds
//! a case where panicking is permitted, so the panic condition is their union. A
//! function with no `#[may_panic(...)]` clause cannot panic at all (the empty union,
//! `false`, is the default).
//!
//! `#[ensures(...)]` now only describes the normal-return outcome, and is again
//! closed under logical equivalence (it no longer knows about panicking).
//!
//! Verified properties:
//!
//! +   Functions may only panic in states satisfying their panic condition
//! +   Callers either prove the callee cannot panic here, or propagate the
//!     panic to their own panic condition

extern crate creusot_std;
use creusot_std::prelude::*;

/// May panic unconditionally; but if it returns, it returns the first element.
#[may_panic(true)]
#[ensures(result == l@[0])]
pub fn first(l: &[u8]) -> u8 {
    l[0]
}

/// Panics exactly when `i` is out of bounds.
#[may_panic(i@ >= l@.len())]
// function effectively panics when i >= l.len
#[ensures(i@ < l@.len())]
#[ensures(result == l@[i@])]
pub fn get(l: &[u8], i: usize) -> u8 {
    l[i]
}

/// A function that cannot panic can still call `get`: it must prove that the
/// panic condition of `get` cannot hold for these arguments.
#[requires(l@.len() > 0)]
#[ensures(result == l@[0])]
pub fn get_first_safe(l: &[u8]) -> u8 {
    get(l, 0)
}

/// Panics of the callee propagate to the panic outcome of the caller.
#[may_panic(l@.len() == 0)]
#[ensures(result == l@[0])]
pub fn first_or_panic(l: &[u8]) -> u8 {
    get(l, 0)
}

/// Explicit `panic!` is allowed when the panic condition holds.
#[may_panic(x@ >= 100)]
#[ensures(result == x)]
pub fn check_limit(x: u32) -> u32 {
    if x >= 100 { panic!("x is too large") } else { x }
}

/// Note: arithmetic overflow is still modeled as a hard proof obligation (a
/// precondition of the arithmetic operations), even in functions that may
/// panic. Here the panic case is made explicit instead.
#[may_panic(x == u32::MAX)]
#[ensures(result@ == x@ + 1)]
pub fn increment(x: u32) -> u32 {
    if x == u32::MAX { panic!("overflow") } else { x + 1 }
}

/// The function returns `x / y`, or panics (only possible when `y == 0`).
#[may_panic(y@ == 0)]
#[ensures(result@ == x@ / y@)]
pub fn div(x: u32, y: u32) -> u32 {
    x / y
}

/// Two distinct panic *causes*, expressed as one `#[may_panic]` clause each. The
/// clauses are disjoined, so the function may panic when `y == 0` (division by
/// zero) *or* when `x == u32::MAX` (explicit `panic!`) — the panic condition is
/// their union `y@ == 0 || x == u32::MAX`.
///
/// This is exactly the case that distinguishes disjunction from conjunction: a
/// conjunctive reading would give the panic condition `y@ == 0 && x == u32::MAX`,
/// under which the division-by-zero panic (reachable with `y == 0` and `x` free)
/// could not be discharged, and the function would fail to verify.
#[may_panic(y@ == 0)]
#[may_panic(x == u32::MAX)]
#[ensures(result@ == x@ / y@)]
pub fn div_two_causes(x: u32, y: u32) -> u32 {
    if x == u32::MAX { panic!("x saturated") } else { x / y }
}

/// A function which always panics: it may panic anywhere (`#[may_panic(true)]`) and
/// never returns normally (`#[ensures(false)]`). Note how the two concerns are
/// now stated independently, where the old `#[ensures(panics)]` coupled them.
#[may_panic(true)]
#[ensures(false)]
pub fn always_panic() -> u8 {
    panic!("boom")
}

/// Calling an always-panicking function is fine in a function that may panic:
/// the callee's `#[ensures(false)]` makes the caller's normal exit unreachable.
#[may_panic(true)]
#[ensures(false)]
pub fn call_always_panic() -> u8 {
    always_panic()
}

/// Since an always-panicking function never returns, any normal postcondition
/// holds vacuously — here `result@ == 42` even though nothing returns `42`.
#[may_panic(true)]
#[ensures(result@ == 42)]
pub fn vacuous_normal_post() -> u8 {
    panic!("boom")
}

/// The same, one call away: the callee never returns, so the caller's normal
/// exit is unreachable and its (arbitrary) postcondition holds vacuously.
#[may_panic(true)]
#[ensures(result@ == 42)]
pub fn vacuous_normal_post_call() -> u8 {
    always_panic()
}

/// Declared faillible, but with an unsatisfiable panic condition: the function
/// is therefore forced to never actually panic.
#[may_panic(false)]
#[ensures(result@ == 24)]
pub fn never_panic() -> u8 {
    24
}

/// A non-panicking function can call `never_panic`: it must discharge
/// `never_panic`'s panic condition (`false`) at the call site, which is trivial.
#[ensures(result@ == 24)]
pub fn never_panic_call() -> u8 {
    never_panic()
}

/// Mixed case: a normal-only postcondition (`result@ <= 100`) alongside a panic
/// clause and another normal postcondition. `#[ensures]` constrains only the
/// returning outcome, `#[may_panic]` only the panic outcome — they stay orthogonal.
#[ensures(result@ <= 100)]
#[may_panic(x@ > 100)]
#[ensures(result == x)]
pub fn clamp_or_panic(x: u32) -> u32 {
    if x > 100 { panic!("too large") } else { x }
}

// ---------------------------------------------------------------------------
// `#[panics(P)]` sugar: the function panics *exactly* when `P` (bi-conditional).
// It desugars to `#[may_panic(P)]` (panic ⟹ P) + `#[ensures(!P)]` (P ⟹ panic).
// Contrast with `#[may_panic(P)]`, which only bounds panics (panic ⟹ P).
// ---------------------------------------------------------------------------

/// Panics if and only if `i` is out of bounds. The `i@ < l@.len()` part of the
/// normal-return postcondition is generated by the sugar (`#[ensures(!(i@ >= …))]`).
#[panics(i@ >= l@.len())]
#[ensures(result == l@[i@])]
pub fn get_iff(l: &[u8], i: usize) -> u8 {
    l[i]
}

/// `#[panics(true)]` = "always panics": desugars to `#[may_panic(true)]` plus
/// `#[ensures(!true)] = #[ensures(false)]`, so the normal exit is unreachable.
#[panics(true)]
pub fn always_panic_sugar() -> u8 {
    panic!("boom")
}

/// `#[panics(false)]` = "faillible but never actually panics": the generated
/// `#[ensures(!false)] = #[ensures(true)]` is trivial and the unsatisfiable panic
/// condition forces the body not to panic.
#[panics(false)]
#[ensures(result@ == 7)]
pub fn never_panic_sugar() -> u8 {
    7
}

/// Multiple `#[panics(Pi)]` combine disjunctively: panics iff `y == 0` OR
/// `x == u32::MAX`. Beyond `div_two_causes` (which used `#[may_panic]`), the sugar
/// also guarantees the converse of each cause on normal return (`y != 0` and
/// `x != u32::MAX`) — that is the point of the bi-conditional.
#[panics(y@ == 0)]
#[panics(x == u32::MAX)]
#[ensures(result@ == x@ / y@)]
pub fn div_two_causes_iff(x: u32, y: u32) -> u32 {
    if x == u32::MAX { panic!("x saturated") } else { x / y }
}
