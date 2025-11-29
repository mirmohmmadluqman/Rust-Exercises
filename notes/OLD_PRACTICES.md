# main_old_not_formatted:
```
// #![allow(unused)]

// fn main() {
//     loop {
//         println!("Logop");
//                 println!("Loopx");
//                 println!("Loxgop");
//                 println!("Loodp");
//                 println!("Lcoop");
//                 println!("Lofop");
//                 println!("Loop");                println!("Loop");
//                 println!("Loop");
//                 println!("Loop");                println!("Loop");
//                 println!("Loop");
//                 println!("Loop");                println!("Loop");
//                 println!("Loop");
//                 println!("Loop");                println!("Loop");
//                 println!("Loop");
//                 println!("Loop");
//                 println!("Lolop");
//                 println!("Laoop");
//                 println!("Lopop");
//                 println!("Lofop");
//                 println!("Loop");
//                 println!("Lofop");
//                 println!("Locop");
//                 println!("Lboop");                println!("Loop");
//                 println!("Loop");
//                 println!("Loop");                println!("Loop");
//                 println!("Loop");
//                 println!("Loop");                println!("Loop");
//                 println!("Loop");
//                 println!("Loop");
//                 println!("Lo2op");                println!("Loop");
//                 println!("Loop");
//                 println!("Loop");                println!("Loop");
//                 println!("Loop");
//                 println!("Loop");
//                 println!("Loop");
//                 println!("Loop");                println!("Loop");
//                 println!("Loop");
//                 println!("Loop");                println!("Loop");
//                 println!("Loop");
//                 println!("Loop");                println!("Loop");
//                 println!("Loop");
//                 println!("Loop");
//                 println!("Loop");
//                 println!("Loop");
//                 println!("Loop");
//                 println!("Lo3op");
//                 println!("Ltoop");
//                                 println!("Lo321op");
//                 println!("Ltoop");                println!("Loop");
//                 println!("Loop");
//                 println!("Loop");                println!("Loop");
//                 println!("Loop");
//                 println!("Loop");                println!("Loop");
//                 println!("Loop");
//                 println!("Loop");
//                 println!("Loeop");
//                 println!("Lo2dop");
//                 println!("Lodop");                println!("Loop");
//                 println!("Loop");
//                 println!("Loop");
//                 println!("Looa21p");
//                 println!("Loogp");
//                 println!("Loop");
//                 println!("Loop");                println!("Loop");
//                 println!("Loop");
//                 println!("Loop");
//                 println!("Loop");
//                 println!("Loop");
//                 println!("L21ooup");
//                 println!("Loeop");                println!("Loop");
//                 println!("Loop");
//                 println!("Loop");
//                 println!("L21odop");
//                 println!("Lodop");
//                 println!("Loop");                println!("Loop");
//                 println!("Loop");
//                 println!("Loop");
//                 println!("Loop");                println!("Loop");
//                 println!("Loop");
//                 println!("Loop");                println!("Loop");
//                 println!("Loop");
//                 println!("Loop");                println!("Loop");
//                 println!("Loop");
//                 println!("Loop");
//                 println!("Loop");
//                 println!("Loop");
//                 println!("Looap");
//                 println!("Loogp");
//                 println!("Looup");
//                                 println!("Loop");
//                 println!("Loop");
//                 println!("Loop");                println!("Loop");
//                 println!("Loop");
//                 println!("Loop");
//                 println!("Loop");
//                 println!("Louop");
//                 println!("Loiop");
//                 println!("Lo21op");
//                 println!("Lotop");
//                 println!("Loop");
//                 println!("Loop");                println!("Loop");
//                 println!("Loop");
//                 println!("Loop");                println!("Loop");
//                 println!("Loop");
//                 println!("Loop");
//                 println!("Loop");
//                 println!("Loop");
//                 println!("Loqop");
//                 println!("Loqop");
        
//     }
// }

/* */
#![allow(unused)]

fn main() {
    println!(" ");
    let mut i = 0;

    println!("-------Loop directly; working on break ---------");
    loop {
        i += 1;
        println!("i = {}", i);
        if i > 5{
            break;
        }
    }

    println!("------------------While loop -------------------");
    let mut i = 0;
    while i <= 5{
        i += 1;
        println!("i = {}", i);
    }

    println!("------------------For loop -------------------"); 
    let arr = [1, 2, 3, 4, 5];
    let n: usize = arr.len();
    for i in 0..n{ // Or instead use 0..=5, 0..6
        println!("i = {}", arr[i]);
    };  


    println!("------------------For loop with enumerate -------------------");
    let v = vec![1, 2, 3, 4, 5];

    for m in 0..n{ // N = 5 ;  
    println!("i = {}", arr[m]);
    };  

    println!("------------------For loop but .... -------------------");
    for (i, value) in arr.iter().enumerate() {
    println!("Index: {}, Value: {}", i, value);
    }



    println!("------------------ Test -------------------");
                        /* Code not working */
    // let nums: Vec<i32> = vec![1, 2, 3, 4, 5];
    // numms = 0;                      // ❌ Not declared (no let mut)
    // loop {
    //     nnums = nums.iter().sum()   // ❌ Missing semicolon + types mismatch
    //     break;
    // }
    // println!("Sum = {}", nnums);    // ❌ nnums might not exist yet


    let nums: Vec<i32> = vec![1, 2, 3, 4, 5];
    let mut nnums: i32; // variable to store the sum
    loop {
        nnums = nums.iter().sum();
        break; // stop after one run
    }
    println!("Sum = {}", nnums);

}
```

# main_old:
```
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
```