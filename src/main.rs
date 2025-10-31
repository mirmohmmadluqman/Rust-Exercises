#![allow(unused)]

fn main() {
    // Match
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("other"),
    }

    match x {
        1 | 2 => println!("one or two"),
        _ => println!("other"),
    }
}