    #![allow(unused)]
    // #[derive(Debug/*, PartialEq */)]


    // This will panic at runtime, because of overflow and say Too much!
    // fn main() {
    //     // let mut eth = u32::MAX;
    //     // eth += 1;
    //     // println!("ethereum: {eth}")
    //
    //     let x = 4_000_000_000u32.checked_add(1_000_000_000);// Or you can say checked_add(4_000_000_000, 1_000_000_000)
    //     if x.is_none() { println!("Too much!"); } // Happens here!
    // }



    // This will not panic, because we are using checked_add which returns an Option
    // fn main() {
    //     let a: u32 = 10;
    //     let b: u32 = 20;
    //     let safe = a.checked_add(b);      // Some(30)
    //     let overflow = u32::MAX.checked_add(1); // None
    //
    //     println!("{:?}", safe);      // Some(30)
    //     println!("{:?}", overflow);  // None
    // }


    // Tuples in Rust ___________________________________________________________________________________________________________
    // fn return_many() ->(u32, bool) { 
    //     (1u32, true)
    // } 
    //
    // fn main() {
    //     println!("--------------------------------------------------------------------------");
    //
    //     // Normal tuple
    //     println!("Normal tuple:");
    //     let mut a_pex: (bool, char, i32) = (true, 'a',  1);
    //     println!("  {:?}", a_pex.1);
    //
    //     // Empty tuple, also called unit type not uint
    //     println!("Empty tuple, also called unit type not uint:");
    //     let b_pex = ();
    //     println!("  {:?}", b_pex);
    //
    //     // Nested tuple
    //     println!("Nested tuple:");
    //     let c_pex: ((i32, i32), (f32, f32)) = ((1, 2), (3.0, 4.0));
    //     println!("  {:?}", c_pex.1.0); // or you can say (c_pex.1).0
    //
    //     // Destructuring a tuple
    //     println!("Destructuring a tuple:");
    //     let (d_pex, e_pex, f_pex) = a_pex;
    //     println!("  d_pex: {d_pex}, e_pex: {e_pex}, f_pex: {f_pex}");
    //
    //     // Partial destructuring
    //     println!("Partial destructuring:");
    //     let(_, e_pex, _) = a_pex;
    //     println!("  e_pex is {e_pex}");
    //
    //     // Accessing tuple elements using indexing
    //     println!("Accessing tuple elements using indexing:");
    //     let e_pex = a_pex.1;
    //     println!("  e_pex is {e_pex}"); 
    //
    //     // Function returning multiple values using tuple
    //     println!("Function returning multiple values using tuple:");
    //     let (y, z) = return_many();
    //     println!("  y: {y}, z: {z}");
    //
    //     println!("--------------------------------------------------------------------------");
    // }


    // // Arrays in Rust    ___________________________________________________________________________________________________________
    // fn main() {
    //     println!("--------------------------------------------------------------------------");
    //
    //     println!("Arrays in Rust:");
    //     let arr: [i32; 6] = [1, 2, 3, 4, 5, 0];  
    //     println!("  Arr is now {:?}", arr);
    //
    //     let arr = [1, 2, 3, 4, 5];             
    //     println!("  Arr is now: {:?}", arr);
    //
    //     let arr = [0; 5];                      
    //     println!("  Arr is now: {:?}", arr);
    //
    //     println!(" When we print Legth of Array with `arr.len()`: {}", arr.len());
    //
    //     println!("Real Example:");
    //     // Example 1: Game board (3x3 grid)
    //     let board: [[u32; 3]; 3] = [
    //         [1, 2, 3],
    //         [4, 5, 6],
    //         [7, 8, 9],
    //     ];
    //     println!("  Board is now: {:?}", board);
    //
    //     // Example 2: Scores of 5 players
    //     let scores: [u32; 5] = [85, 90, 78, 92, 88];
    //     println!("  Scores are now: {:?}", scores);
    //
    //     // Example 3: RGB color values (all same type)
    //     let rgb: [u8; 3] = [255, 128, 64];
    //     println!("  RGB values are now: {:?}", rgb);
    //
    //     println!("--------------------------------------------------------------------------");
    // }




    // Slices in Rust ___________________________________________________________________________________________________________
    // fn sum_numbers(nums: &[i32]) -> i32 {
    // nums.iter().sum() // Sum all numbers in the slice
    // }
    //
    //
    // fn main() {
    // println!("--------------------------------------------------------------------------");
    // println!("Slices in Rust:");
    //
    //
    // let my_array = [5, 10, 15, 20, 25]; 
    //
    // // Create a slice (reference to part of array)
    // let my_slice = &my_array[1..4];    // [10, 15, 20] Their sum is 45
    //
    // // Pass the slice to function
    // let result = sum_numbers(my_slice);
    // println!("{}", result);            // 45
    //
    // println!("--------------------------------------------------------------------------");
    // }




    // // String: in Rust _____________________________________________________________________________________________________________
    //
    // fn main() {
    //     println!("--------------------------------------------------------------------------");
    //     println!("String in Rust:");
    //
    //     let msg: String = String::from("Hello, Jeff!"); 
    //     println!("{}", msg);
    //
    //     let msg: String = "Hello, Jeff!".to_string(); 
    //     println!("{}", msg);
    //
    //     let length: usize = msg.len();
    //     println!("Length of the string: {}", length);
    //
    //     println!("--------------------------------------------------------------------------");
    //
    //
    //     //Exercise Test
    //     let name = "Jeff";
    //     let greeting = format!("Hi Dear, {name}!");
    //     println!("{}", greeting);
    //
    //     // part 3 of that
    //     let s mut = "Hello";
    //
    //     // let s = s
    //     let s = s.to_string();
    //     format!("{s}!")
    //     println!("{}", s);
    // }




    // // Enum in Rust _____________________________________________________________________________________________________________
    // enum Command {
    //     Play,
    //     Stop,
    //     Skip(u32),
    //     Back(u32),
    //     Resize {width: u32, height: u32}
    // }
    //
    //
    // fn main() {
    //     println!("---------------------------------------------------------------------------");
    //     println!("Enum in Rust:");
    //
    //     let cmd: Command = Command::Play;
    //         println!("{:?}", cmd);
    //     let cmd: Command = Command::Skip(10);
    //         println!("{:?}", cmd);
    //     let cmd: Command = Command::Resize {width: 100, height: 50}; // We can't say Command::Resize(100, 50), it will give error and we do this in tuple! 
    //         println!("{:?}", cmd);
    //
    //     let cmd0: Command = Command::Play;
    //     let cmd1: Command = Command::Skip(10);
    //     println!("Is `cmd0` equal to `cmd1`: {:?}", cmd0 == cmd1);
    //
    //     // Option<T> = some(T) | none {{{Revis it later #revise1553 #bookmark1553 #revise #bookmark}}}
    //     let x: Option<i32> = Some(10);
    //     println!("Is `x` equal to `Some(10)`: {:?}", x == Some(10));
    //     let x: Option<i32> = None;
    //     println!("Is `x` equal to `None`: {:?}", x == None);
    //
    //     // Result<T, E> = Ok(T) | Err(E) {{{Revis it later #revise1553 #bookmark1553 #revise #bookmark}}}
    //     // "100" -> 100
    //     let x: Result<i32, String> = Ok(200);
    //     // ... Just revise it later || @1553x
    //
    //
    //     println!("---------------------------------------------------------------------------");
    // }




    /* // Struct in Rust _____________________________________________________________________________________________________________
    // Struct
    struct Point{
        x: i32,
        y: i32,
    }   

    struct Point3D(i32, i32, i32); // Tuple Struct

    struct Empty; // Empty Struct

    #[derive(Debug)]
    struct Circle{
        radius: u32,
        center: Point,
    }

    fn main() {
        println!("---------------------------------------------------------------------------");
        println!("Struct in Rust:");

        let p = Point {x: 1, y: 1};
        println!("  Point p is at: ({:?})", p);

        println!("  When we print them as _._(p.x): {} {}", p.x, p.y);

        let p = Point3D(-1, 1, 0);
        println!("  This is how we print Tuple Struct, ");
        println!("      Point3D: ({}, {}, {})", p.0, p.1, p.2);

        let empty = Empty;
        
        let circle = Circle {
            radius: 10,
            center: Point { // It is same as `center: Point {x: 0, y: 0},`
                x: 0, 
                y: 0},
        };
        println!("  Normal Debug Method: {:?}", circle);
        println!("  Debug with #: {:#?}", circle);

        // Shorcut
        let x: i32 = 1;
        let y: i32 = 1;
        let p = Point {x: x, y: y}; // When variable name is same we can do this like x, y
        println!("  Shortcut when variable and field name are same: ({:?})", p);
        let p = Point {x, y};
        println!("  Shortcut when variable and field name are same: ({:?})", p);
        
        // Copy fields
        let p0 = Point{x: 0, y: 0};
        let p1 = Point{x: 1, y: p0.y}; // 1 way to copy field
        let p2 = Point{y: 2, ..p0};     // another way to copy field
        println!("  {:?}", p2);

        // update
        let mut p = Point{x: 0, y: 0};
        p.x += 1;
        println!("  Update point: {:?}", p);


        println!("---------------------------------------------------------------------------");
    } */

    // 

    // Vector in Rust _____________________________________________________________________________________________________________
    fn main() {
        let mut v: Vec<i32> = Vec::new();
        v.push(1);
        v.push(2);
        v.push(3);
        println!("Vector v is now: {:?}", v);
    
        let v = vec![1, 3, 3];

        // Option<&i8>
        // Index valid => Some(&val)
        // Index invalid => None
        println!("v[1] is: {:?}", v.get(1)); // Some(&3)
        println!("v[10] is: {:?}", v.get(10)); // None, so func will not panic


        // Exercise Testing
        let x: u32 = 10;
        let y = x;
        let z = x;

            let mut tensor: Vec<u32> = Vec::new();
            tensor.push(x); // or vec![x, y, z]
            tensor.push(y);
            tensor.push(z);
        println!("Tensor is now: {:?}", tensor);
    } 
