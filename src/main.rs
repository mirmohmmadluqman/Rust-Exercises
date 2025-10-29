#![allow(unused)]

use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert("red".to_string(), 10);
    scores.insert("blue".to_string(), 50);
    println!("{:#?}", scores);
    println!("------------------------- Get:");
    

    // Get
    let score: Option<&i32> = scores.get("red");
    println!("Red Score: {:?}", score);
    println!("------------------------- Update:");

    // Update
    let score: &mut i32 = scores.entry("black".to_string()).or_insert(0);
    *score += 25;   // *score: Dereferences the mutable reference to access the actual value (0).

    let score: Option<&i32> = scores.get("black");
    println!("Black Score: {:?}", score);
    

    println!("--------------------------------------------------------------------------------------");

    let mut scorestype2: HashMap<i32, &str> = HashMap::new();
    scorestype2.insert( 1231, "Red");
    scorestype2.insert( 9843, "Blue");
    println!("{:#?}", scorestype2);
    println!("---");

    // Get
    scorestype2.get(&1);
    println!("{:#?}", scorestype2.get(&1));
    println!("---");
    println!("{:#?}", scorestype2.get(&1231));




}
