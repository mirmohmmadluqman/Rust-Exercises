pub fn num_to_string(num: u32) -> String {
    match num {
        // 0 => "zero".to_string(),
        // 1 => "one".to_string(),
        // 2 => "two".to_string(),
        // 3 => "three".to_string(),
        // 4 => "four".to_string(),
        // 5 => "five".to_string(),
        // 6 => "six".to_string(),
        // 7 => "seven".to_string(),
        // 8 => "eight".to_string(),
        // 9 => "nine".to_string(),
        // _ => "many".to_string(),
        
        // or, simpler:

        0 => String::from("zero"),
        1 => String::from("one"),
        2 => String::from("two"),
        3 => String::from("three"),
        _ => String::from("other"),
    }
}

pub fn unwrap_or_default(x: Option<u32>, v: u32) -> u32 {
    match x {
        Some(value) => value,  // extract and return the value
        None => v,             // return default if None
    }
}
