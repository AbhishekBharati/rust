use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_num: u32 = rand::thread_rng().gen_range(1, 101);

    println!("Our secret Number is {}", secret_num);

    loop {
        println!("Please input your guess.");
        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please enter a valid number");

        println!("You guessed : {}", guess);

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too Small!!!"),
            Ordering::Greater => println!("Too Big!!!"),
            Ordering::Equal => {
                println!("You Win!!");
                break;
            }
        }
    }
}
