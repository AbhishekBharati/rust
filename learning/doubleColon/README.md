# There are 2 conditions where we'll be needing :: to access elements.
## 1. Namespace Resolution.
   - Rust organizes code into modules to keep things structured and to avoid name conflicts. Modules can contain ** functions, types, constants, and even other modules **. The `::`, syntax allows you to navigate through these modules to access the items inside them.

## 2. Associated Functions and Constants.
   - In Rust, types can have functions and constants that are not associated with them, but these functions/constants are not tied to a specific instance of the type. They belong to the type itself, not to an instance of that type.
   `fn main() {
      let s = String::new();
      println!("The string is : {}", s);
   }`
   - `String` is a type in Rust's standard library that represents a dynamically sized string.
   - `new` is a function that belong to the `String` type itself, not to any particular instance of `String`.
   - This function is used to create a new, empty `String`.
