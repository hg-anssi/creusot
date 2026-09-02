// WHY3PROVE
extern crate creusot_std;
use creusot_std::prelude::*;

#[may_panic(true)]
#[ensures(false)]
pub fn always_panic() -> u8 {
    panic!("boom")
}

// should declare that the function may panic
pub fn may_panic() -> u8 {
    always_panic()
}
