// WHY3PROVE ENABLE_PANICS_ONLY
extern crate creusot_std;
use creusot_std::{
    ghost::invariant::Tokens,
    prelude::*,
    std::thread::{self, JoinHandleExt},
};

// A caller that is not allowed to panic cannot `join_unwrap` a handle whose spawned
// closure may panic. `join_unwrap` carries `#[may_panic(self.may_panic_on_join())]`,
// so calling it here forces discharging `!may_panic_on_join()`. But the closure
// panics unconditionally (`#[may_panic(true)]`), so `spawn` cannot rule out
// `may_panic_on_join()` — the panic obligation of `join_unwrap` is unprovable.
//
// This is a proof failure: the translation succeeds, but the generated goal at the
// `join_unwrap` call site cannot be proved.
pub fn join_unwrap_unhandled_panic() {
    let f = #[may_panic(true)]
    |_tokens: Ghost<Tokens>| -> u32 { panic!("boom") };
    let j = thread::spawn(f);
    let _ = j.join_unwrap();
}
