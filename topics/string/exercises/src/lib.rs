pub fn hello() -> String {
    "Hello Rust".to_string();
}

pub fn greet(name: &str) -> String {
    let greeting = format!("Hi Dear, {name}!");
    greeting
    
}

pub fn append(mut s: String) -> String {
    // format!("{s} !") I am getting error

    s.push('!');
    s
}


/*                  Done     @mirmohmmadluqman        */