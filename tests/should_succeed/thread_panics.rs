//! Panic contracts on the `creusot_std::std::thread` wrappers.
//!
//! `thread::spawn` lets its closure panic: the panic is caught by the child thread
//! and only resurfaces at `join_unwrap`, tracked by the handle's `may_panic_on_join`
//! predicate. A closure proven panic-free makes `spawn` guarantee
//! `!may_panic_on_join()`, so `join_unwrap` becomes provably panic-free even in a
//! caller that is itself not allowed to panic.

extern crate creusot_std;
use creusot_std::{
    ghost::invariant::Tokens,
    prelude::*,
    std::thread::{self, JoinHandleExt},
};

/// Expressing a thread that *may* panic. The spawned closure is allowed to panic;
/// the panic surfaces at `join_unwrap` (via `may_panic_on_join`) and is *propagated*
/// to this caller's own panic outcome through `#[may_panic]`.
#[may_panic(true)]
pub fn may_panic_thread_propagates() {
    let f = #[may_panic(true)]
    |_tokens: Ghost<Tokens>| -> u32 { panic!("boom") };
    let j = thread::spawn(f);
    let _ = j.join_unwrap();
}

/// Expressing (and proving) that a thread does *not* panic, and that the no-panic
/// guarantee flows all the way through `join_unwrap`. This caller carries no
/// `#[may_panic]`, yet it can `join_unwrap` the handle: since the closure is
/// panic-free, `spawn` guarantees `!may_panic_on_join()`, which discharges
/// `join_unwrap`'s panic condition. The closure's postcondition flows through too.
#[ensures(result@ == 42)]
pub fn no_panic_flows_through() -> u32 {
    let f = |_tokens: Ghost<Tokens>| -> u32 { 42 };
    let j = thread::spawn(f);
    j.join_unwrap()
}

#[ensures(result@ == 42)]
pub fn scoped_thread() -> u32 {
    thread::scope(|s| {
        let f = |_tokens: Ghost<Tokens>| -> u32 { 42 };
        let j = s.spawn(f);
        j.join_unwrap()
    })
}
