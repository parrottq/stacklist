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

struct StackListIter<'a, T>(Option<&'a Node<'a, T>>); // TODO: Necessary? Make it not copy?

impl<'a, T> Iterator for StackListIter<'a, T> {
    // Directly walkable a good idea? No `.iter()`? When copy it is hard to use?
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

struct E<'a, T> {
    // previous: Option<&'a mut E<'a, T>>,
    previous: Option<&'a E<'a, T>>,
    value: T,
}

// struct StackList<'a, 'b, T> {
//     last_node: &'a mut Option<&'b mut Node<'b, T>>,
// }
// fn minimize1<'a, T>(node: &'a mut Node<'a, T>) -> ()
// where
//     T: Default,
// {
//     {
//         let mut node_inner = E {
//             previous: node,
//             value: Default::default(),
//         };
//         {
//             let e = &node_inner;
//             {
//                 minimize(Some(e))
//             }
//         }
//     }
// }

struct Pointer<'a, T>(&'a mut T);

fn a3<'a, 'b, T>(a: Pointer<'a, T>, b: &'b mut T) -> &'b mut T
where
    'a: 'b,
{
    let f: Pointer<'b, T> = a;
    f.0
}

fn a2<'a, 'b, T>(a: &'a mut T, b: &'b mut T) -> &'b mut T
where
    'a: 'b,
{
    let f: Pointer<'b, T> = Pointer::<'a, T>(a);
    f.0
}

fn a1<'a, 'b, T>(a: &'a mut T, b: &'b mut T) -> &'b mut T
where
    'a: 'b,
{
    let a1: &'a mut T = a;
    let a2: &'b mut T = a1;
    a2
}

fn minimize<'a, T>(node: Option<&'a E<'a, T>>) -> ()
where
    T: Default,
{
    {
        let mut node_inner = E {
            previous: node,
            value: Default::default(),
        };
        {
            let e = &node_inner;
            {
                minimize(Some(e))
            }
        }
    }
}

// fn start_list<'a, 'b, T, U>(
fn start_list<'a, T, U>(
    mut fun: impl for<'c> FnMut(StackList<'c, T>) -> Op<T, U>,
    // mut fun: impl for<'a_1, 'a_2> FnMut(&mut StackList<'a_1, 'a_2, T>) -> Op<T, U>,
    node: Option<&'a Node<'a, T>>,
) -> U
// where
//     'a: 'b,
    // fn start_list<'a, T, U>(
    //     mut fun: impl FnMut() -> Op<T, U>,
    //     // mut fun: impl for<'a_1, 'a_2> FnMut(&mut StackList<'a_1, 'a_2, T>) -> Op<T, U>,
    //     node: Option<&'a mut Node<'a, T>>,
    // ) -> U
{
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

fn start_list_empty<T, U>(
    mut fun: impl for<'c> FnMut(StackList<'c, T>) -> Op<T, U>,
    // fun: impl for<'a_1, 'a_2> FnMut(&mut StackList<'a_1, 'a_2, T>) -> Op<T, U>,
) -> U {
    start_list(fun, None)
    // match fun() {
    //     Op::Store(store_val) => {
    //         let mut node_inner = Node {
    //             previous: None,
    //             value: store_val,
    //         };
    //         start_list(fun, &mut node_inner)
    //     }
    //     Op::Return(return_val) => return_val,
    // }
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
