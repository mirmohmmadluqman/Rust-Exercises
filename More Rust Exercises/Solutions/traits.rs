// ============================================
// EXAMPLE: Understanding Traits
// ============================================

// This example shows how traits work with a simple "Describe" trait.
// Study this BEFORE attempting the exercise above.

// STEP 1: Define a trait (contract)
trait Describe {
    fn description(&self) -> String;
}

// STEP 2: Define structs
struct Book {
    title: String,
    author: String,
}

struct Movie {
    title: String,
    director: String,
}

// STEP 3: Implement the trait for Book
impl Describe for Book {
    fn description(&self) -> String {
        // format! creates a String
        // No semicolon = automatic return
        format!("Book: {} by {}", self.title, self.author)
    }
}

// STEP 4: Implement the trait for Movie
impl Describe for Movie {
    fn description(&self) -> String {
        format!("Movie: {} directed by {}", self.title, self.director)
    }
}

// STEP 5: Generic function using the trait
fn print_description<T: Describe>(item: &T) {
    // <T: Describe> means:
    // - T is a generic type
    // - T MUST have the Describe trait
    // - So we can call item.description()
    println!("{}", item.description());
}

fn main() {
    // Create instances
    let book = Book {
        title: String::from("1984"),
        author: String::from("George Orwell"),
    };

    let movie = Movie {
        title: String::from("Inception"),
        director: String::from("Christopher Nolan"),
    };

    // Direct calls
    println!("{}", book.description());
    // Output: Book: 1984 by George Orwell

    println!("{}", movie.description());
    // Output: Movie: Inception directed by Christopher Nolan

    // Generic function calls
    print_description(&book);
    // Output: Book: 1984 by George Orwell

    print_description(&movie);
    // Output: Movie: Inception directed by Christopher Nolan
}

// ============================================
// Key Concepts:
// ============================================
// 1. Trait = Contract (must implement specified methods)
// 2. impl Trait for Type = "This type has the trait"
// 3. <T: Trait> = Generic type that MUST have the trait
// 4. &self = Reference to the current instance
// 5. format! = Creates a String (no semicolon = return)