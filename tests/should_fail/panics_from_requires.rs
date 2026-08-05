// WHY3PROVE ENABLE_PANICS_ONLY
extern crate creusot_std;
use creusot_std::prelude::*;

#[requires(x@ != 0)]
pub fn fn_with_req(x: u8) {}

// transforming requires clause into may_panic clause is not legal
#[may_panic(x@ == 0)]
pub fn req_into_panic(x: u8) {
    fn_with_req(x)
}
