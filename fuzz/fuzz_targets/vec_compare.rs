#![no_main]

use libfuzzer_sys::fuzz_target;
use stacklist::{new_list, Op, StackListIter};

// TODO: Mutable compare
// TODO: ZST compare?

fuzz_target!(|data: Vec<Op<(), ()>>| {
    // if data.len() < 10 {
    //     return;
    // }
    if data.len() > 100 {
        return; // Truncate
    }

    let mut unique_id = 0i32;
    let data: Vec<Op<i32, i32>> = data
        .into_iter()
        .map(|x| {
            let result = match x {
                Op::Store(_) => Op::Store(unique_id),
                Op::Return(_) => Op::Return(unique_id),
                Op::Pop => Op::Pop,
                Op::PopMultiple(e) => Op::PopMultiple(e),
                Op::Clear => Op::Clear,
            };
            unique_id += 1;
            result
        })
        .collect();
    // println!("{data:?}");

    let mut iter = data.iter();
    let mut reference: Vec<i32> = vec![];
    let mut vec_return = None;
    let stack_return: i32 = new_list(|lst| {
        {
            if false {
                let mut ref_iter = reference.iter().rev();
                let mut lst_iter: StackListIter<i32> = lst.iter();
                let ref_col = ref_iter.collect::<Vec<_>>();
                let lst_col = lst_iter.collect::<Vec<_>>();
                assert_eq!(ref_col, lst_col, "{:?} {:?}", ref_col, lst_col);
            }

            let mut ref_iter = reference.iter().rev();
            let mut lst_iter: StackListIter<i32> = lst.iter();
            loop {
                match (ref_iter.next(), lst_iter.next()) {
                    (Some(&a), Some(&b)) => {
                        assert_eq!(a, b);
                    }
                    (None, None) => break,
                    (a, b) => assert_eq!(a, b),
                }
            }
        }

        if let Some(elem) = iter.next() {
            match elem {
                Op::Store(elem) => reference.push(*elem),
                Op::Pop => {
                    reference.pop();
                }
                Op::PopMultiple(i) => {
                    for _ in 0..(*i).min(reference.len()) {
                        reference.pop();
                    }
                }
                Op::Clear => reference.clear(),
                Op::Return(vec_return_inner) => {
                    vec_return = Some(*vec_return_inner);
                }
            }
            *elem
        } else {
            vec_return = Some(0);
            Op::Return(0)
        }
    });

    assert_eq!(vec_return.unwrap(), stack_return);
});
