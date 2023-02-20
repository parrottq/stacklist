#![feature(generators)]
use stacklist::{list_from_fn, list_from_generator, yield_op, Op, StackListToken};
use std::pin::Pin;

fn list() {
    let mut i = 0i32;

    let result = list_from_fn(|lst| {
        i += 1;
        match i {
            0..=4 => Op::Store(Box::new(i)),
            5 => Op::PopMultiple(2),
            _ => Op::Return(lst.iter_mut().map(|x| *x.as_ref()).sum::<i32>()),
        }
    });

    println!("Total: {result}");
}

fn generator() {
    let mut gen = |t: StackListToken<u8, _>| {
        let mut f = Some(t);

        yield_op!(f, Op::Store(0));
        yield_op!(f, Op::Store(20));

        for i in 0..10 {
            yield_op!(f, Op::Store(i));
        }

        let t = f.take().unwrap();

        let total = t
            .borrow()
            .iter()
            .copied()
            .map(Into::<u16>::into)
            .sum::<u16>();
        return (t, total);
    };
    let result = list_from_generator(Pin::new(&mut gen), || {});
    println!("Total: {result}");
}

fn main() {
    list();
    generator();
}
