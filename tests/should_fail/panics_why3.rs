// WHY3PROVE
extern crate creusot_std;
use creusot_std::prelude::*;

// should declare that the function may panic
#[ensures(true)]
pub fn may_panic() -> u8 {
    panic!("boom")
}
