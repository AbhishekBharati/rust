mod math {
    // This are the two ways to return a function :-
    pub fn add(a : i32, b : i32) -> i32 {
        return a + b;
    }
    pub fn sub(a : i32, b : i32) -> i32 {
        a - b
    }
}

fn main() {
    let sum = math::add(5,3);
    let diff = math::sub(10,40);

    println!("Sum : {}", sum);
    println!("Diff : {}", diff);

    let s = String::new();
    println!("The string is: {}", s);
}
