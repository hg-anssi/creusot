use crate::prelude::*;

extern_spec! {
    impl bool {
        #[check(ghost)]
        #[ensures(result == if self { Some(t) } else { None })]
        fn then_some<T>(self, t: T) -> Option<T> {
            if self { Some(t) } else { None }
        }

        #[requires(self ==> f.precondition(()))]
        #[ensures(!self ==> result == None)]
        #[ensures(self ==>
            match result {
                None => false,
                Some(t) => f.postcondition_once((), t)
            }
        )]
        #[may_panic(self && f.panic_condition(()))]
        fn then<T, F: FnOnce() -> T>(self, f: F) -> Option<T> {
            if self { Some(f()) } else { None }
        }
    }
}
