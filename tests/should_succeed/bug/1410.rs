extern crate creusot_std;
use creusot_std::prelude::*;

#[warn(let_underscore_drop)]
#[requires(f.precondition(()))]
#[may_panic(f.panic_condition(()))]
pub fn bar<F: FnMut()>(mut f: F) {
    let f_snap = snapshot!(f);
    #[invariant(produced.len() == 0 ==> f.precondition(()))]
    #[invariant(produced.len() == 0 ==> f.panic_condition(()) == f_snap.panic_condition(()))]
    for _ in 0..1 {
        f();
    }
}
