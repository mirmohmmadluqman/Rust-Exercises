/*   ------------------                          Hash Map                          ------------------   * /


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




} */

/*   ------------------                          If else                           ------------------   */

#![allow(unused)]

fn main() {
    let mut x: i32 = 0;

    let error_detected: &str = "Your code is changed, broken or has any logical error(flaw).";

    let onlyowner: i32 = if x >=     0 {
        println!("x is positive");
        x 
    } else if x < -1 {
        x = -10;
        println!("x is now negative");
        x 
    } else {
        println!("{}", error_detected);
        error_detected.len() as i32
    };

    println!("onlyowner = {}", onlyowner);
}
