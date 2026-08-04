//! A ghost function cannot panic — this test *uses* that fact in proof position.
//!
//! It exercises both shapes the fact takes:
//!   1. a **normal** (concrete) `#[check(ghost)]` closure, whose `panic_condition`
//!      already unfolds definitionally to `false`;
//!   2. a **generic** ghost function `F: FnGhost`, whose `panic_condition` stays
//!      opaque and can only be discharged via the `FnGhost` no-panic law.
//!
//! Both callers must discharge the `#[requires(!f.panic_condition((x,)))]` of the
//! `assert_wont_panic` sink (a stand-in for a real no-panic API such as
//! `thread::Scope::spawn` or the iterator adapters).
//!
//! EXPECTED STATUS relative to `user-stories/todo/USER_STORY-fnghost-no-panic-law.md`:
//!   - the concrete case (1) proves today;
//!   - the generic case (2) is UNPROVABLE under `--enable-panics` BEFORE the US, and
//!     becomes provable AFTER the law is added. So the `.with_panics` proof of this
//!     file is expected to FAIL before the US and PASS after.
//!   - without `--enable-panics`, `panic_condition ≡ false`, so both cases are trivial.

extern crate creusot_std;
use creusot_std::{ghost::FnGhost, prelude::*};

/// A no-panic sink: it only accepts a closure whose panic condition is false at `x`.
/// It does not call `f` (so it stays panic-free itself); it just returns it. Callers
/// must *prove* `!f.panic_condition((x,))` to call it.
// #[requires(!f.panic_condition((x,)))]
// pub fn assert_wont_panic<F: Fn(i32) -> i32>(f: F, x: i32) -> F {
//     f
// }

/// Case 1 — normal (concrete) ghost function: `panic_condition` folds to `false`,
/// so discharging the sink's precondition succeeds without the law.
pub fn concrete_ghost_no_panic() {
    let f = #[check(ghost)]
    |x: i32| x + 1;
    proof_assert!(forall <n> !f.panic_condition(n))
    // let _ = assert_wont_panic(f, 1);
}

/// Case 2 — generic ghost function: `F` is only known to be `FnGhost`, so
/// `panic_condition` is opaque here. Discharging the sink's precondition requires
/// the `FnGhost` no-panic law. Unprovable (ON mode) before the US, provable after.
pub fn generic_ghost_no_panic<F: Fn(i32) -> i32 + FnGhost>(f: F) {
    proof_assert!(forall <n> !f.panic_condition(n))
    // let _ = assert_wont_panic(f, 1);
}
