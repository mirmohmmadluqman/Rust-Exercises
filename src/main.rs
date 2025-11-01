#![allow(unused)]

fn main() {
    // // Match
    // let x = 1;
    // match x {
    //     1 => println!("one"),
    //     2 => println!("two"),
    //     _ => println!("other"),
    // }

    // match x {
    //     1 | 2 => println!("one or two"),
    //     _ => println!("other"),
    // }

    let x: Option<i32> = Some(10);
    match x {
        Some(i @ 1..=10) => println!("{}", i),
        _ => println!("other"),
    }

    let res: Result<i32, &str> = Ok(5);
    match res {
        Ok(val) => println!("ok: {}", val),
        Err(e) => println!("error: {}", e),
    }


    println!("----------------------------------------------------------------------");
    let x: Option<i32> = Some(10);
    let x: Option<i32> = None;
    let z: i32 = match x {
        Some(val) => val,
        None => 0,
    }; 
    println!("z: {}", z);
}   