use stacklist::{new_list, Op};

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