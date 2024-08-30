// Lifetimes is an very important concept. The idea is rust mei jo yeh check krta hai ki koi bhi
// variable dangling pointer hai ya nhi woh borrower hota hai.

// Ab borrower ko kaise pta ki agar koi function jo ek string ka reference le rha hai and whi
// return kr rha hai... woh dangling pointer nhi kr rha, Here Lifetimes comes into the picture.

use std::fmt::Display;

// Lifetimes annotations in struct :-
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn retur_part(&self, announcement: &str) -> &str {
        println!("Attention Please... {}", announcement);
        self.part
    }
}

// Let's sum up all the concepts that is generics, traits and lifetimes :-
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement... {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");

    {
        let string2 = String::from("xyz");
        let result = longest(&string1, &string2);
        println!("The longest string is {}", result);
        // Here the lifetime of result is equal to lifetime of string2 coz string2's lifetime is
        // less than string1's lifetime.
    }

    let first_sentence = novel.split('.').next().expect("Couldn't find .");
    let novel = String::from("Call me nodix... Some years ago...");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

// 1st problem :- x and y both could have different lifetimes, toh ab yeh return value ko kiska
// reference assign hoga woh nhi pta so isiliye we'll be seeing an error of lifetimes
// 2nd Problem :- we don't know exact lifetimes of x and y coz this are just placeholders.
// --> So x and y both have different lifetimes so we need to know which one is return lifetime.

// --> So here generic annotation lifecycle come into the picture.
// It relates the lifetimes of multiple references.

// --> We created our lifetime and x & y used them & our return type also uses the same lifetime.

// &i32         --> A reference
// &'a i32      --> A reference with explicit lifetime.
// &'a mut i32  --> A mutable reference with an explicit lifetime.

// In this example it simply means that x, y & return value will have same lifetime.
// (They don't actually change the lifetime they just create relations among different references.)
// So here if x has smaller lifetime then y then return values lifetime will be equal to x's
// lifetime and vice versa.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// This are the cases when compiler will get the lifetimes automatically :-
// --> There are some interesting rules in defining lifetime :- (Lifetime Elision)
// 1. Each parameter that is reference get it's own lifetime parameter.
// 2. If there is exactly 1 i/p lifetime parameter, that lifetime is assigned to all output
//    lifetime parameters.
// 3. If there are multiple input lifetime parameters, but one of them is &self or &mut self, then
//    the lifetime of the self is assigned to all o/p lifetime parameters.
