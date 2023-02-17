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

pub mod callback;
pub mod list;
pub mod node_mut;

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "alloc", derive(Arbitrary))]
pub enum Op<T, U> {
    Store(T),
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

#[derive(Clone, Copy)]
pub struct StackList<'a, T>(Option<&'a Node<'a, T>>); // TODO: Necessary? Make it not copy?
                                                      // TODO: Store length? (benchmark)

impl<'a, T> StackList<'a, T> {
    pub fn iter(&self) -> StackListIter<'a, T> {
        StackListIter(self.0)
    }
}

pub struct StackListIter<'a, T>(Option<&'a Node<'a, T>>);

impl<'a, T> Iterator for StackListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            Some(inner) => {
                self.0 = inner.previous;
                Some(&inner.value)
            }
            None => None,
        }
    }
}

struct Node<'a, T> {
    previous: Option<&'a Node<'a, T>>,
    value: T,
}

fn inner_stack_list<'a, T, U>(
    fun: &mut impl for<'c> FnMut(StackList<'c, T>) -> Op<T, U>,
    node: Option<&'a Node<'a, T>>,
) -> OpResult<U>
// where
//     T: Debug,
//     U: Debug,
{
    match fun(StackList(node)) {
        Op::Store(store_val) => {
            let node_inner = Node {
                previous: node,
                value: store_val,
            };
            loop {
                return match inner_stack_list(fun, Some(&node_inner)) {
                    OpResult::PopMultiple(count) => {
                        if let Some(count) = count.checked_sub(1) {
                            return OpResult::PopMultiple(count);
                        }
                        continue; // Too many pops shoud panic?
                    }
                    OpResult::Return(result) => OpResult::Return(result),
                    OpResult::Clear => OpResult::Clear,
                };
            }
        }
        Op::Return(return_val) => OpResult::Return(return_val),
        Op::Clear => OpResult::Clear,
        Op::Pop => OpResult::PopMultiple(1),
        Op::PopMultiple(count) => OpResult::PopMultiple(count), // TODO: Many pops should panic?
    }
}

pub fn new_list<T, U>(mut fun: impl for<'c> FnMut(StackList<'c, T>) -> Op<T, U>) -> U
// where
//     T: Debug,
//     U: Debug,
{
    loop {
        match inner_stack_list(&mut fun, None) {
            OpResult::Return(result) => return result,
            OpResult::PopMultiple(_) => (), // TODO: Too many pops should panic?
            OpResult::Clear => (),
        }
    }
}

// TODO: usize -> ()?
pub struct StackListToken<T, U>(*const usize, PhantomData<T>, PhantomData<*const U>);

impl<'a, T, U> StackListToken<T, U> {
    pub unsafe fn new(stack_list: &StackList<'a, T>) -> Self {
        let e = stack_list as *const StackList<'a, T>;
        let e = e as *const usize;
        Self(e, PhantomData, PhantomData)
    }

    pub fn lifetimeless_view<R, F>(stack_list: &StackList<'a, T>, fun: F) -> R
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

    pub fn borrow(&'a self) -> StackListTokenBorrowed<'a, T> {
        let inner: &'a StackList<T> = unsafe { &*(self.0 as *const StackList<'a, T>) };
        StackListTokenBorrowed(inner)
    }
}

#[derive(Clone, Copy)]
pub struct StackListTokenBorrowed<'a, T>(&'a StackList<'a, T>);

impl<'a, T> Deref for StackListTokenBorrowed<'a, T> {
    type Target = StackList<'a, T>;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

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
    new_list(|lst| {
        StackListToken::lifetimeless_view(&lst, |tok| {
            let result = fun.as_mut().resume(tok);
            match result {
                GeneratorState::Yielded((token, op)) => (token, op),
                GeneratorState::Complete((token, result)) => (token, Op::Return(result)),
            }
        })
    })
}
