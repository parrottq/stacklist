#![feature(generators)]
use std::pin::Pin;

use stacklist::{list_from_generator, yield_op, Op, StackListToken};

fn main() {
    let mut gen = |t: StackListToken<u8, _>| {
        let mut f = Some(t);

        yield_op!(f, Op::Store(0));
        yield_op!(f, Op::Store(0));

        for i in 0..10 {
            yield_op!(f, Op::Store(i));
        }

        let t = f.take().unwrap();
        dbg!(t.borrow().iter().collect::<Vec<_>>());

        return (t, 0u16);
    };
    let e = Pin::new(&mut gen);
    let result = list_from_generator(e, || {});

    // TODO: Macro to make unique closure for each generator
    // TODO: Document unsafe
    // TODO: Check unsafe with MIRI
    // TODO: Documentation generally
    // TODO: Cleanup public interface (look at rustdoc)
    // TODO: Make nightly generator support optional
    dbg!(result);
}
