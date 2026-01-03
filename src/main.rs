#![allow(unused)]

fn take(s: String) {
    println!("{s}");
}

fn /*takeWithoutMovingOwnership*/borrow(s: &String) {
    println!("borrowed: {s}");
}

fn xMain() {
    let mut s = String::from("rust");
    let s0 = s.clone();

    let s1 = &mut s;

    println!("Popping elements from the string:");
    while let Some(c) = s1.pop() {
        println!("Removed: {c}");
    }

    println!("After we popped... s(s1) = [{}]", s1);

    s1.push_str(&s0);
    println!("After we pushed s0... s(s1) = [{}]", s1);

    s1.push_str("lang");
    println!("After we pushed lang... s(s1) = [{}]", s1);

    println!("Final result: [{}]", s1);

    println!("Result via s directly: {}]", s);
}

fn main() {
    // xMain();

    let s = String::from("hello");
    borrow(&s);
}