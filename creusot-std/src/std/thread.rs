use crate::{ghost::invariant::Tokens, prelude::*};
use ::std::thread::{self, JoinHandle, ScopedJoinHandle};

/// Extension trait for [`JoinHandle`].
pub trait JoinHandleExt<T> {
    /// Predicate that specifies the valid return results for the handle.
    #[logic(prophetic)]
    fn valid_result(self, x: T) -> bool;

    /// Whether joining this handle may observe a panic propagated from the spawned
    /// closure. `spawn` sets this from the closure's `panic_condition`: a closure
    /// proven not to panic yields `false` here, which makes `join_unwrap` panic-free.
    #[logic(opaque, prophetic)]
    fn may_panic_on_join(self) -> bool {
        dead
    }

    /// This function is a wrapper `self.join().unwrap()`.
    ///
    /// It panics iff the spawned thread panicked; that possibility is tracked by
    /// [`may_panic_on_join`](Self::may_panic_on_join). When the spawned closure is
    /// proven not to panic, `may_panic_on_join` is `false` and this call is panic-free.
    // NOTE: This is a way to avoid ::std::thread::Result, which:
    //  - contains a dyn;
    //  - we don't know how to handle the Err case in Creusot.
    #[may_panic(self.may_panic_on_join())]
    #[ensures(self.valid_result(result))]
    fn join_unwrap(self) -> T;
}

impl<T> JoinHandleExt<T> for JoinHandle<T> {
    #[logic(opaque, prophetic)]
    fn valid_result(self, _x: T) -> bool {
        dead
    }

    #[may_panic(self.may_panic_on_join())]
    #[ensures(self.valid_result(result))]
    #[trusted]
    fn join_unwrap(self) -> T {
        self.join().unwrap()
    }
}

impl<T> JoinHandleExt<T> for ScopedJoinHandle<'_, T> {
    #[logic(opaque, prophetic)]
    fn valid_result(self, _x: T) -> bool {
        dead
    }

    #[may_panic(self.may_panic_on_join())]
    #[ensures(self.valid_result(result))]
    #[trusted]
    fn join_unwrap(self) -> T {
        self.join().unwrap()
    }
}

extern_spec! {
    impl<T> JoinHandle<T> {
        #[ensures(true)] // no spec, but you can call this if you want
        fn is_finished(&self) -> bool;
    }

    impl<T> ScopedJoinHandle<'_, T> {
        #[ensures(true)] // no spec, but you can call this if you want
        fn is_finished(&self) -> bool;
    }
}

/// Creusot wrapper around [`std::thread::spawn`].
///
/// The only difference is that the closure gives access to a fresh token object
///
/// Parameter `f` is allowed to panic: the panic is caught by the child thread and only
/// resurfaces when the handle is joined via [`join_unwrap`](JoinHandleExt::join_unwrap),
/// which is why the returned handle carries `may_panic_on_join` whenever `f` may panic.
#[requires(forall<t: Ghost<Tokens>> (forall<ns> t.contains(ns)) ==> f.precondition((t,)))]
#[ensures(exists<t: Ghost<Tokens>> (forall<ns> t.contains(ns))
    && (forall<r> result.valid_result(r) ==> f.postcondition_once((t,), r))
    && (result.may_panic_on_join() ==> f.panic_condition((t,)))
)]
#[trusted]
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
where
    F: FnOnce(Ghost<Tokens>) -> T + Send + 'static,
    T: Send + 'static,
{
    ::std::thread::spawn(|| f(Tokens::new()))
}

/// Creusot's replacement for [`Scope`].
pub struct Scope<'scope, 'env: 'scope> {
    inner: &'scope thread::Scope<'scope, 'env>,
}

impl<'scope, 'env: 'scope> Scope<'scope, 'env> {
    #[requires(forall<t: Ghost<Tokens>> (forall<ns> t.contains(ns)) ==> f.precondition((t,)))]
    // Parameter f is not allowed to panic (unlike std::thread::spawn) because panicking threads
    // will trigger delayed panic at the end of the lifetime of the scope if not "caught" before by using ScopedJoinHandle::join().
    #[requires(forall<t: Ghost<Tokens>> (forall<ns> t.contains(ns)) ==> !f.panic_condition((t,)))]
    #[ensures(exists<t: Ghost<Tokens>> (forall<ns> t.contains(ns))
        && (forall<r> result.valid_result(r) ==> f.postcondition_once((t,), r))
        // Since f.panic_condition((t,)) is required to be false, equiv to (result.may_panic_on_join() ==> f.panic_condition((t,)))
        && (!result.may_panic_on_join())
    )]
    #[trusted]
    pub fn spawn<F, T>(&mut self, f: F) -> ScopedJoinHandle<'scope, T>
    where
        F: FnOnce(Ghost<Tokens>) -> T + Send + 'scope,
        T: Send + 'scope,
    {
        self.inner.spawn(|| f(Tokens::new()))
    }
}

/// Creusot wrapper around [`std::thread::scope`].
#[requires(forall<s> inv(s) ==> f.precondition((s,)))]
#[may_panic(exists<s> inv(s) && f.panic_condition((s,)))]
#[ensures(exists<s> inv(s) && f.postcondition_once((s,),result))]
#[trusted]
pub fn scope<'env, F, T>(f: F) -> T
where
    F: for<'scope> FnOnce(&mut Scope<'scope, 'env>) -> T,
{
    ::std::thread::scope(|s| f(&mut Scope { inner: s }))
}
