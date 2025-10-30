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

}
