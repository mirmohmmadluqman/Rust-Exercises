// ============================================
// TRAIT EXERCISE: Animal Sounds
// ============================================

// PART 1: Define a trait called `Animal`
// - The trait should have ONE method: `make_sound(&self) -> String`
// - This method should return a String representing the animal's sound

// PART 2: Create TWO structs:
// - `Dog` with a field `name: String`
// - `Cat` with a field `name: String`

// PART 3: Implement the `Animal` trait for BOTH structs:
// - Dog's make_sound should return: "{name} says: Woof!"
// - Cat's make_sound should return: "{name} says: Meow!"

// PART 4: Write a generic function called `let_animal_speak`
// - It should accept ANY type that implements the `Animal` trait
// - It should print the result of calling `make_sound()`

// PART 5: In main(), create:
// - A Dog named "Buddy"
// - A Cat named "Whiskers"
// - Call `let_animal_speak` for both animals

// ============================================
// YOUR CODE HERE:
// ============================================

fn main() {
    // TODO: Create a Dog and a Cat
    // TODO: Call let_animal_speak for both
}

// TODO: Define the Animal trait


// TODO: Define the Dog struct


// TODO: Define the Cat struct


// TODO: Implement Animal for Dog


// TODO: Implement Animal for Cat


// TODO: Write the generic function let_animal_speak


// ============================================
// Expected Output:
// ============================================
// Buddy says: Woof!
// Whiskers says: Meow!