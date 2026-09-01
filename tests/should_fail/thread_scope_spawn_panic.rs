// WHY3PROVE ENABLE_PANICS_ONLY
extern crate creusot_std;
use creusot_std::{ghost::invariant::Tokens, prelude::*, std::thread};

// `Scope::spawn` forbids panicking closures: it `requires(!f.panic_condition(..))`,
// because a panicking scoped thread would raise a *delayed* panic at the end of the
// scope (unlike `thread::spawn`, whose panic is caught and only resurfaces at
// `join_unwrap`). Passing a may-panic closure to `Scope::spawn` therefore makes that
// precondition unprovable.
//
// This is a proof failure: the translation succeeds, but the `!panic_condition`
// obligation at the `s.spawn(..)` call site cannot be proved.
#[may_panic(true)]
pub fn scope_spawn_rejects_panic() {
    thread::scope(|s| {
        let f = #[may_panic(true)]
        |_tokens: Ghost<Tokens>| -> u32 { panic!("boom") };
        let _ = s.spawn(f);
    });
}
