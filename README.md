
# Stacklist

Dynamically storing values without allocating. Stacklist internally works by calling a function recursively to allocate on the stack.

## Example
```rust
use stacklist::{list_from_fn, Op};

fn main() {
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
```

You can also use a generator for better ergonomics on nightly.
```rust
#![feature(generators)]
use std::pin::Pin;

use stacklist::{list_from_generator, yield_op, Op, StackListToken};

fn main() {
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
```