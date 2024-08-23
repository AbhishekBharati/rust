# Learning Ownerships :-

## Ownership Rules :-
 - Each value in Rust has an Owner.
 - There can be only one owner at a time.
 - When the owner goes out of the scope, the value will be dropped.

``` 
{                     // s is not valid here, it's not yet declared. 
    let s = "hello";   // s is valid from this point.
    // do stuff with s here.
} //This scope is over and now s is no longer valid here.
```

 - Normal DataTypes are okay, coz when the scope is finished the variables will be popped out of stack.
 But, our area of Interest is what happens with the space that is dynamically allocated how does rust manages that?

 - So here comes to parts. 
 1. The memory must be requested by the memory allocator at the run time.
 2. We need a way of returning this memory to the allocator when we're done with our String.

 That first part is done by us: when we call String::from, it's implementation requests the memory it needs.
This is pretty much universal in programming languages.

But while freeing up the space rusts took a different path: The memory is automatically returned when the variable that owns it goes out of the scope. When a variable goes out of the scope, Rust calls a special function for us. This function is called, `drop`. And it's where the author of String can puts the code to return the memory. Rust calls the drop automatically at the closing curly brackets.

Something like shallow copy is happening here, but only thing is we can't access the previous owner of the data.
But there is way to Deeply copy the data, i.e. not only the one that is visible in stack but also the one that is visible in heap.
Can do it via :-
`let s1 = String::from("Hello");
let s2 = s1.clone();
// Here we deeply copied the data that means s1 refers to different location and s2 refers to different location so we can access both of them.`

## Ownerships and Functions :-
- When we are passing a variable(who is storing an address that is pointing to a memory location on heap) inside a function as a parameter. The rust has to uninitialize it coz as the scope of that function is called `drop` will be called. Now if the variable is uninitialized in main block then as tha main block's scope will end Rust will again call the  `drop` function which will try to remove a memory allocation that doesn't exists.
** So once we pass it to a function we can't use it.**

But if the variable holds a value in the stack itself, then it doesn't matter we can use it after we pass it to a function.

```
fn main() {
    let s = String::from("hello"); // s comes into the scope
    takes_ownership(s); // s's value moves into the function...
                        // ...and so s is no longer valid here
    let x = 5; // x comes to scope.
    makes_copy(x);      // x would move into the function,
                        // but i32 is Copy, so it's ok to still use x afterward.
}// Here, x goes out of the scope and then s. But s's value was moved, so nothing special happens.
fn takes_ownership(some_string: String){// Some string comes into the scope.
  println!(some_string);
}// Here, some_string goes out of the scope and drop function is called. The backing memory is freed.
fn makes_copy(some_int : i32){ // some_integer comes into the scope
  println!(some_int);
}// Here, some_int goes out of the scope. Nothing special happens.
```

## Return Values and Scope :-
**Returning Values can also transfer ownerships.**

```
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}
```

When a variable that includes data on heap goes out of the scope, the value will be cleaned up `drop` unless ownership of the data is moved to another variable.

While this works but taking ownerships and returning ownerships with every function is a bit annoying. What if we want to take a function the value but not the ownership? It's quite annoying that anything we pass in also needs to be passed back if we want to use it again, in addition to any data resulting from the body of the function that we might want to return as well.

Rust does let us return multiple values as a tuple.

```
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
```

**But this is too much hectic. Luckily for us rust has a feature for using a value without transferring ownership, called references.**

