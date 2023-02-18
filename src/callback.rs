use crate::{list::StackList, node_mut::NodeMut, Op, OpResult};

fn inner_stack_list<'a, 'b, T, U>(
    fun: &mut impl for<'c, 'd> FnMut(&mut StackList<'c, 'd, T>) -> Op<T, U>,
    node: Option<&'b mut NodeMut<'a, T>>,
) -> OpResult<U> {
    let mut stack = StackList::new(node);
    match fun(&mut stack) {
        Op::Store(store_val) => {
            let node_inner = &mut NodeMut::new(stack.take(), store_val);
            loop {
                return match inner_stack_list(fun, Some(node_inner)) {
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

pub fn new_list<T, U>(mut fun: impl for<'c, 'd> FnMut(&mut StackList<'c, 'd, T>) -> Op<T, U>) -> U {
    loop {
        match inner_stack_list(&mut fun, None) {
            OpResult::Return(result) => return result,
            OpResult::PopMultiple(_) => (), // TODO: Too many pops should panic?
            OpResult::Clear => (),
        }
    }
}
