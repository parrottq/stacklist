use std::fmt::Debug;
use arbitrary::Arbitrary;

#[derive(Clone, Copy, Debug, Arbitrary)]
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
