//! State monad

use std::rc::Rc;

use crate::{Applicative, Functor, Hkt1, Id, Magmoidal, Monad, Monoidal};

/// `State` wraps a function `S -> (S, A)`.
///
/// The function consumes the state and produces a new state and a value.
///
/// # Example
///
/// ```rust
/// use cats_core::*;
/// use std::rc::Rc;
///
/// #[derive(Debug, Clone, PartialEq, Eq)]
/// enum TrunstileState {
///     Locked,
///     Unlocked,
/// }
///
/// #[derive(Debug, Clone, PartialEq, Eq)]
/// enum TrunstileOutput {
///     Thank,
///     Open,
///     Tut,
/// }
///
/// let coin_s = State::new(Rc::new(|_| {
///     (TrunstileState::Unlocked, TrunstileOutput::Thank)
/// }));
/// let push_s = State::new(Rc::new(|s| match s {
///     TrunstileState::Locked => (TrunstileState::Locked, TrunstileOutput::Tut),
///     TrunstileState::Unlocked => (TrunstileState::Locked, TrunstileOutput::Open),
/// }));
///
/// assert_eq!(
///     coin_s.run(TrunstileState::Locked),
///     (TrunstileState::Unlocked, TrunstileOutput::Thank)
/// );
/// assert_eq!(push_s.eval(TrunstileState::Locked), TrunstileOutput::Tut);
/// assert_eq!(push_s.exec(TrunstileState::Locked), TrunstileState::Locked);
///
/// let monday_s = coin_s.flat_map(move |a1| {
///     let push_s = push_s.clone();
///     State::new(Rc::new(move |s| {
///         let (s, a2) = push_s.run(s);
///         (s, (a1.clone(), a2))
///     }))
/// });
/// assert_eq!(
///     monday_s.run(TrunstileState::Locked),
///     (
///         TrunstileState::Locked,
///         (TrunstileOutput::Thank, TrunstileOutput::Open)
///     )
/// );
/// ```
#[derive(Clone)]
pub struct State<S, A>(Rc<dyn Fn(S) -> (S, A)>);

impl<S, A> State<S, A>
where
    S: Clone,
{
    /// Create a new `State`
    pub fn new(f: Rc<dyn Fn(S) -> (S, A)>) -> Self {
        Self(f)
    }

    /// Run the `State`
    pub fn run(&self, s: S) -> (S, A) {
        (self.0)(s)
    }

    /// Run and give back the result of the `State`
    pub fn eval(&self, s: S) -> A {
        self.run(s).1
    }

    /// Run and give back the new state of the `State`
    pub fn exec(&self, s: S) -> S {
        self.run(s).0
    }

    /// Set the state to `s`
    ///
    /// The name `put` is from Haskell's `Control.Monad.State`.
    pub fn put(&self, s: S) -> State<S, ()>
    where
        for<'a> S: 'a,
    {
        State::new(Rc::new(move |_| (s.clone(), ())))
    }

    /// Get the state without changing it
    pub fn get(&self) -> State<S, S> {
        State::new(Rc::new(move |s: S| (s.clone(), s.clone())))
    }
}

impl<S, A> Hkt1 for State<S, A> {
    type Unwrapped = A;
    type Wrapped<T> = State<S, T>;
}

impl<S, A> Functor for State<S, A>
where
    for<'a> S: Clone + 'a,
    for<'a> A: Clone + 'a,
{
    fn map<B, F>(self, f: F) -> State<S, B>
    where
        for<'a> F: Fn(A) -> B + 'a,
    {
        State::new(Rc::new(move |s| {
            let (s, a) = self.run(s);
            (s, f(a))
        }))
    }
}

impl<S, A> Magmoidal for State<S, A>
where
    for<'a> S: Clone + 'a,
    for<'a> A: 'a,
{
    fn product<B>(self, b: State<S, B>) -> State<S, (A, B)>
    where
        for<'a> B: 'a,
    {
        State::new(Rc::new(move |s| {
            let (s, a) = self.run(s);
            let (s, b) = b.run(s);
            (s, (a, b))
        }))
    }
}

impl<S, A> Monoidal for State<S, A>
where
    for<'a> S: Clone + 'a,
    for<'a> A: 'a,
{
    fn unit() -> State<S, ()> {
        State::new(Rc::new(|s| (s, ())))
    }
}

impl<S, A> Applicative for State<S, A>
where
    for<'a> S: Clone + 'a,
    for<'a> A: Clone + 'a,
{
    fn pure<B>(b: B) -> State<S, B>
    where
        Self: Id<State<S, B>>,
        for<'a> B: Clone + 'a,
    {
        State::new(Rc::new(move |s| (s, b.clone())))
    }

    fn ap<B, F>(self, ff: Self::Wrapped<F>) -> Self::Wrapped<B>
    where
        for<'a> F: Fn(Self::Unwrapped) -> B + 'a,
    {
        State::new(Rc::new(move |s| {
            let (s, f) = ff.run(s);
            let (s, a) = self.run(s);
            (s, f(a))
        }))
    }
}

impl<S, A> Monad for State<S, A>
where
    for<'a> S: Clone + 'a,
    for<'a> A: Clone + 'a,
{
    fn flat_map<B, F>(self, f: F) -> State<S, B>
    where
        for<'a> F: Fn(A) -> State<S, B> + 'a,
    {
        State::new(Rc::new(move |s| {
            let (s, a) = self.run(s);
            f(a).run(s)
        }))
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::*;

    #[test]
    fn test_state() {
        #[derive(Debug, Clone, PartialEq, Eq)]
        enum TrunstileState {
            Locked,
            Unlocked,
        }

        #[derive(Debug, Clone, PartialEq, Eq)]
        enum TrunstileOutput {
            Thank,
            Open,
            Tut,
        }

        let coin_s = State::new(Rc::new(|_| {
            (TrunstileState::Unlocked, TrunstileOutput::Thank)
        }));
        let push_s = State::new(Rc::new(|s| match s {
            TrunstileState::Locked => (TrunstileState::Locked, TrunstileOutput::Tut),
            TrunstileState::Unlocked => (TrunstileState::Locked, TrunstileOutput::Open),
        }));

        assert_eq!(
            coin_s.run(TrunstileState::Locked),
            (TrunstileState::Unlocked, TrunstileOutput::Thank)
        );
        assert_eq!(push_s.eval(TrunstileState::Locked), TrunstileOutput::Tut);
        assert_eq!(push_s.exec(TrunstileState::Locked), TrunstileState::Locked);

        let monday_s = coin_s.flat_map(move |a1| {
            let push_s = push_s.clone();
            State::new(Rc::new(move |s| {
                let (s, a2) = push_s.run(s);
                (s, (a1.clone(), a2))
            }))
        });
        assert_eq!(
            monday_s.run(TrunstileState::Locked),
            (
                TrunstileState::Locked,
                (TrunstileOutput::Thank, TrunstileOutput::Open)
            )
        );
    }
}
