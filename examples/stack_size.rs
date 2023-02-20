use stacklist::{list_from_fn, Op};

fn inner<T>(value: T) -> String
where
    T: Clone,
{
    let mut i = 0;
    let result = list_from_fn(|lst| {
        let r = match i {
            0 => Op::Store(value.clone()),
            1 => Op::Store(value.clone()),
            _ => {
                let mut iter = lst.iter();
                Op::Return((
                    iter.next().unwrap() as *const T as usize,
                    iter.next().unwrap() as *const T as usize,
                ))
            }
        };
        i += 1;
        r
    });
    format!("{}-{}={}", result.1, result.0, result.1 - result.0)
}

fn main() {
    dbg!(inner(()));
    dbg!(inner(0u8));
    dbg!(inner(0u64));
    dbg!(inner([0u8; 1]));
    dbg!(inner([0u8; 2]));
    dbg!(inner([0u8; 4]));
    dbg!(inner([0u8; 8]));
    dbg!(inner([0u8; 9]));
    dbg!(inner([0u8; 15]));
    dbg!(inner([0u8; 16]));
    dbg!(inner([0u8; 32]));
    dbg!(inner([0u8; 64]));
}
