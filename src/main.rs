use std::num::NonZeroUsize;

enum Op<T, U> {
    Store(T),
    Return(U),
    Pop,
    PopMultiple(usize),
    Clear,
}

enum OpResult<U> {
    Return(U),
    // Pop,
    PopMultiple(NonZeroUsize),
    Clear,
}

#[derive(Clone, Copy)]
struct StackList<'a, T>(Option<&'a Node<'a, T>>); // TODO: Necessary? Make it not copy?

impl<'a, T> StackList<'a, T> {
    pub fn iter(&self) -> StackListIter<'a, T> {
        StackListIter(self.0)
    }
}

struct StackListIter<'a, T>(Option<&'a Node<'a, T>>);

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
) -> OpResult<U> {
    match fun(StackList(node)) {
        Op::Store(store_val) => {
            let node_inner = Node {
                previous: node,
                value: store_val,
            };
            loop {
                return match inner_stack_list(fun, Some(&node_inner)) {
                    OpResult::PopMultiple(count) => {
                        let count = count.get();
                        if count > 0 {
                            return OpResult::PopMultiple(NonZeroUsize::new(count - 1).unwrap());
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
        Op::Pop => OpResult::PopMultiple(NonZeroUsize::new(1).unwrap()),
        Op::PopMultiple(count) => NonZeroUsize::new(count)
            .map(|x| OpResult::PopMultiple(x))
            .unwrap_or(OpResult::Clear), // TODO: Many pops should panic?
    }
}

fn new_list<T, U>(mut fun: impl for<'c> FnMut(StackList<'c, T>) -> Op<T, U>) -> U {
    loop {
        match inner_stack_list(&mut fun, None) {
            OpResult::Return(result) => return result,
            OpResult::PopMultiple(_) => (), // TODO: Too many pops should panic?
            OpResult::Clear => (),
        }
    }
}

fn main() {
    let mut i = 0i32;
    let result = new_list(|lst| {
        i += 1;
        match i {
            0..=4 => {
                println!("Storing {i}");
                Op::Store(Box::new(i))
            }
            5 => {
                println!(
                    "{}",
                    String::from_iter(lst.iter().map(|d| format!("{d}, ")))
                );
                Op::Pop
            }
            6 => {
                println!(
                    "{}",
                    String::from_iter(lst.iter().map(|d| format!("{d}, ")))
                );
                Op::PopMultiple(2)
            }
            7 => {
                println!(
                    "{}",
                    String::from_iter(lst.iter().map(|d| format!("{d}, ")))
                );
                Op::Store(Box::new(i))
            }
            _ => {
                println!(
                    "{}",
                    String::from_iter(lst.iter().map(|d| format!("{d}, ")))
                );
                println!(
                    "Total {}",
                    lst.iter()
                        .map(|x| {
                            let e: i32 = *x.as_ref();
                            e
                        })
                        .sum::<i32>()
                );
                Op::Return(1)
            }
        }
    });

    println!("{result}");
}
