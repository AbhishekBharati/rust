fn main() {
    let mut temperature : f64 = 0.0;
    let mut choice : String = String::new();

    println!("Temperature Converter");
    println!("Choose an Option : ");
    println!("1: Farenheit to Celcius");
    println!("2: Celcius to Farenheit");

    // Get the user input for the choice.
    std::io::stdin().read_line(&mut choice).expect("Failed to Read Line");

    // Parse the choice to an Integer 
    let choice: u32 = choice.trim().parse().expect("Please enter a number");

    if choice == 1 {
        println!("Enter Temperature in Fahrenheit");

        // Get temperature in Fahrenheit from the user :-
        let mut temp_input = String::new();
        std::io::stdin().read_line(&mut temp_input).expect("Failed to Read Line");

        // Parse the input to f64 :-
        temperature = temp_input.trim().parse().expect("Please enter a valid number");

        // Convert the temperature to Celcius
        temperature = fahrenheitToCelcius(temperature);

        // Display the Result :-
        println!("Temperature in Celcius: {:.2}", temperature);
    } else if choice == 2 {
        println!("Enter temperature in Celcius :");

        // Get the temperature in Celcius from the user :-
        let mut temp_input = String::new();
        std::io::stdin().read_line(&mut temp_input).expect("Failed to Read Line");

        // Parse the input to f64
        temperature = temp_input.trim().parse().expect("Please enter a Valid Number");

        //Conver the temperature to Fahrenheit
        temperature = celciusToFahrenheit(temperature);

        // Display the result :-
        println!("Temperature in Fahrenheit: {:.2}", temperature);
    } else {
        println!("Invalid Choice Please run the program again");
    }
}

fn fahrenheitToCelcius(fahrenheit: f64)->f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn celciusToFahrenheit(celcius: f64)->f64 {
    (celcius * 9.0/5.0) + 32.0
}
