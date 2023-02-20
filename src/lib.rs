#![feature(generator_trait)]
#![cfg_attr(not(feature = "alloc"), no_std)]
use core::{
    fmt::Debug,
    marker::PhantomData,
    ops::{Deref, Generator, GeneratorState},
    pin::Pin,
};

#[cfg(feature = "alloc")]
use arbitrary::Arbitrary;

// TODO: Compare stack size of recursive call (w/ ZST and other types)
// TODO: Create empty list on stack shorthand `with_size(3, Default::default(), |lst| {lst.iter_mut()})` (better name needed)

// TODO: Cleanup public interface
mod callback;
mod list;
mod node_mut;
mod node_ref;

pub use callback::list_from_fn;
pub use list::{StackList, StackListMutIter, StackListIter};

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "alloc", derive(Arbitrary))]
pub enum Op<T, U> {
    Store(T),
    // From iter
    Return(U),
    Pop,
    PopMultiple(usize),
    Clear,
}

enum OpResult<U> {
    Return(U),
    // Pop,
    PopMultiple(usize),
    Clear,
}


// TODO: usize -> ()?
pub struct StackListToken<T, U>(*const usize, PhantomData<T>, PhantomData<*const U>);

impl<'a, 'b, T, U> StackListToken<T, U> {
    pub unsafe fn new(stack_list: &StackList<'b, 'a, T>) -> Self {
        let e = stack_list as *const StackList<'b, 'a, T>;
        let e = e as *const usize;
        Self(e, PhantomData, PhantomData)
    }

    pub fn lifetimeless_view<R, F>(stack_list: &StackList<'b, 'a, T>, fun: F) -> R
    where
        F: FnOnce(StackListToken<T, U>) -> (StackListToken<T, U>, R),
    {
        // TODO: Why is this safe? (unique generic from closure, not clonable, unique type)
        unsafe {
            let result = fun(StackListToken::new(stack_list));
            let _: StackListToken<T, U> = result.0; // Make sure this is returned
            result.1
        }
    }

    pub fn borrow(&'a self) -> StackListTokenBorrowed<'b, 'a, T> {
        let inner: &'a StackList<T> = unsafe { &*(self.0 as *const StackList<'b, 'a, T>) };
        StackListTokenBorrowed(inner)
    }
}

#[derive(Clone, Copy)]
pub struct StackListTokenBorrowed<'a, 'b, T>(&'a StackList<'a, 'b, T>);

impl<'a, 'b, T> Deref for StackListTokenBorrowed<'a, 'b, T> {
    type Target = StackList<'a, 'b, T>;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

// TODO: Make the names better
pub fn list_from_generator<T, R, U>(
    mut fun: Pin<
        &mut impl Generator<
            StackListToken<T, U>,
            Yield = (StackListToken<T, U>, Op<T, R>),
            Return = (StackListToken<T, U>, R),
        >,
    >,
    _unique: U,
) -> R
where
    U: FnOnce(),
{
    callback::list_from_fn(|lst| {
        StackListToken::lifetimeless_view(lst, |tok| {
            let result = fun.as_mut().resume(tok);
            match result {
                GeneratorState::Yielded((token, op)) => (token, op),
                GeneratorState::Complete((token, result)) => (token, Op::Return(result)),
            }
        })
    })
}
