fn main() {
    let mut s = String::from("Hello,"); // When we use this String type we are allocating memory in
                                        // heap.
    s.push_str(" World!!"); // How did we manage to concatenate the string?
                            // Coz, string type allocates the memory in heap where as per our
                            // requirements we can add more space.
    println!("{s}");

    let a = 5;
    let b = a;
    println!("{a}, {b}"); // Here i can still access the a Coz here the size is fixed and values are
                          // known. But this isn't what happens when it comes to string.
    let s1 = String::from("hello");
    let s2 = s1;
    // Now we know that s2 is storing the address that direct towards the heap. So basically both
    // s1 and s2 are refering to the same location.
    // Now the main reason why s1 is not accessible coz as the code goes out of the scope the rust
    // will run drop function for both s1 and s2 which will try to remove the same memory location.
    // So, to ensure memory safety, after the line s2 = s1. Rust consider s1 no longer valid.
    println!("{s2}");
    let str1 = String::from("Hello");
    let str2 = str1.clone();

    println!("str1 : {str1}, str2 : {str2}");
}
