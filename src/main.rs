#[allow(unused_variables)]
#[allow(unused)]



fn main(){
    // Stack
    let x: i32 = 1;
    let arr: [i32; 10] = [1; 10];

    // Heap
    let mut s: String = "hello".to_string();
    s += " world";
    println!("{}", s);

    let mut v = vec![1, 2, 3];
    v.push(4);
    println!("{:?}", v);

}   