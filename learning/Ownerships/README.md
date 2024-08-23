#Learning Ownerships :-

##Ownership Rules :-
 - Each value in Rust has an Owner.
 - There can be only one owner at a time.
 - When the owner goes out of the scope, the value will be dropped.

` {                     // s is not valid here, it's not yet declared. 
    let s = "hello";   // s is valid from this point.
    // do stuff with s here.
} //This scope is over and now s is no longer valid here.`

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

