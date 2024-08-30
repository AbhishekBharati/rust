// We can also create generics with Struct :-

struct Point<T, Y> {
    x: T,
    y: Y,
}

struct Point1<T> {
    x: T,
    y: T,
}

// And ofc we can also create generics with implementation or methods :-
impl<T> Point1<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point1<f64> {
    fn y(&self) -> &f64 {
        &self.y
    }
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    // Generics in Structs :-
    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: 5.0, y: 10.0 };
    let p3 = Point { x: 5.0, y: 10 };
    let p5 = p1.mixup(p2);

    // Generics in Impl or methods :-
    let p4 = Point1 { x: 10, y: 52 };
    p4.x();
    let p = Point1 { x: 10.5, y: 20.3 };
    p.x();
    p.y();

    // Generics in Enum :- (We Already have been using enum in past learning) :-
    enum Option<T> {
        Some(T),
        None,
    }

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    // Generics in Function calls :-
    let number_list = vec![34, 50, 25, 100, 65];

    let largest = get_largest(number_list);

    let char_list = vec!['a', 'z', 'c', 'u', 'k'];
    let largest_char = get_largest(char_list);
    println!("The largest number is {}", largest);
    println!("The largest character is {}", largest_char);
}

// Here the problem is it will only be working for i32 values but what if i want to run the same
// logic for characters?
// Here comes Generics into the picture.

// <T : PartialOrd + Copy> --> This line indicates that the type can only be the one which can be
// ordered and copied.
fn get_largest<T: PartialOrd + Copy>(number_list: Vec<T>) -> T {
    let mut largest = number_list[0];
    for num in number_list {
        if num > largest {
            largest = num;
        }
    }
    largest
}

fn get_largest_number(num_list: Vec<i32>) -> i32 {
    let mut largest = num_list[0];
    for num in num_list {
        if num > largest {
            largest = num;
        }
    }
    largest
}
