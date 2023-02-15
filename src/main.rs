enum Op<T, U> {
    Store(T),
    Return(U),
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

fn start_list<'a, T, U>(
    mut fun: impl for<'c> FnMut(StackList<'c, T>) -> Op<T, U>,
    node: Option<&'a Node<'a, T>>,
) -> U {
    match fun(StackList(node)) {
        Op::Store(store_val) => {
            let node_inner = Node {
                previous: node,
                value: store_val,
            };
            let e = &node_inner;
            start_list(fun, Some(e))
        }
        Op::Return(return_val) => return_val,
    }
}

fn start_list_empty<T, U>(fun: impl for<'c> FnMut(StackList<'c, T>) -> Op<T, U>) -> U {
    start_list(fun, None)
}

fn main() {
    let mut i = 0i32;
    let result = start_list_empty(|lst| {
        i += 1;
        match i {
            0..=4 => {
                println!("Storing {i}");
                Op::Store(Box::new(i))
            }
            _ => {
                let all = String::from_iter(lst.iter().map(|d| format!("{d}, ")));

                println!(
                    "Total {}; {all}",
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
