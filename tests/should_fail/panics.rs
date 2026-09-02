extern crate creusot_std;
use creusot_std::prelude::*;

// A panic condition is a predicate over the inputs: `result` does not exist in
// the panic outcome, so mentioning it is an error (name resolution).
#[may_panic(result@ == 0)]
pub fn panics_mentions_result() -> u8 {
    panic!("boom")
}

// The `#[panics(...)]` sugar surfaces the same errors as its `#[may_panic(...)]`
// half: `result` is not in scope in the panic condition.
#[panics(result@ == 0)]
pub fn panics_sugar_mentions_result() -> u8 {
    panic!("boom")
}
