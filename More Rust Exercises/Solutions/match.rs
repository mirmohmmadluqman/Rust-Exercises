fn main() {
    // call the function with Some and None
    check_buddy(Some(100));
    check_buddy(None);
}

// function that takes Option<u64> as parameter
fn check_buddy(value: Option<u64>) {
    match value {
        Some(_) => println!("yeah buddy lightweight"),
        None => println!("No buddy"),
    }
}


// Explanation:

// - `Option<u64>` means the value can be either Some(number) or None.
// - `match` checks which one it is.
// - `_` ignores the inner number since you just need to know if it’s there.

// --------------------------------------------------------------------------------------------------------------------------------

// If you want to understand it properly, check this:
fn main() {
    let result = divide(10, 0);

    match result {
        Ok(answer) => println!("Answer: {}", answer),
        Err(msg) => println!("Error: {}", msg),
    }
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}

// Why-we-use/What-are results(`Result<...>`):

// Prevents your program from crashing on errors.
// Forces you to handle both success and failure.
// Makes your code safe, clear, and predictable.