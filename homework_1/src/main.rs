// 1. Declare a constant for the freezing point of water in Fahrenheit (32°F)
const FAHREN: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64{
    //(32°F − 32) × 5/9
    (f - FAHREN) * (5.0/9.0)

}

fn celsius_to_fahrenheit(c: f64) -> f64{
    //(0°C × 9/5) + 32
    (c * (9.0/5.0)) + FAHREN
}


fn assignment1(){
    // Temp converter
    // Print the next 5 integer temperatures (e.g., if you start with 32°F, print conversions for 33°F, 34°F, 35°F, 36°F, and 37°F
    println!("Print and convert Fahrenheit to Celcius temperatures!\n");
    
    let mut fahrenheit: f64 = FAHREN;
    println!("Fahrenheit: {}°F -> Celcius: {:.2}°C", fahrenheit,fahrenheit_to_celsius(fahrenheit));

    for i in 1..=5{
        let f = fahrenheit + i as f64;
        println!("Fahrenheit: {}°F -> Celcius: {:.2}°C", f,fahrenheit_to_celsius(f));
    }
}

// 2. Implement a function `is_even(n: i32) -> bool` that returns true if a number is even, false otherwise.
fn is_even(n: i32) -> bool{
    if n % 2 == 0 {return true;} else {return false;}
}

fn assignment2(){
// 1. Create an array of 10 integer numbers of your choice.
    let array = [1,5,2,9,8,6,30,4,7,3];
// 3. Use a for loop to iterate through the array and for each number:
//  - Print whether it's even or odd using your `is_even` function
//  - If the number is divisible by 3, print "Fizz" instead
//  - If the number is divisible by 5, print "Buzz" instead
//  - If it's divisible by both 3 and 5, print "FizzBuzz"
    println!("Fizz, Buzz, FizzBuzz, Even or Odd!");
    for num in array {
        if num % 15 == 0 {
            println!("FizzBuzz");
        } else if num % 3 == 0 {
            println!("Fizz");
        } else if num % 5 == 0 {
            println!("Buzz");
        } else if is_even(num) {
            println!("{} is even", num);
        } else {
            println!("{} is odd", num);
        }
    }
// 4. Use a while loop to find and print the sum of all numbers in the array.
    println!("\nSum of all Numbers!");
    let mut sum = 0;
    let mut x = 0;
    while x < array.len() {
        sum += array[x];
        x += 1;
    }
    println!("The sum of all the numbers is {}.", sum);

// 5. Use a loop to find and print the largest number in the array.
    println!("\nPrint The Largest Number!");
    let mut largest_num = 0;
    for temp in array{
        if temp > largest_num{
            largest_num = temp;
        }
    }
    println!("The largest number is {}\n", largest_num);
}

// Implement a function check_guess(guess: i32, secret: i32) -> i32 that returns:
// 0 if the guess is correct
// 1 if the guess is too high
// -1 if the guess is too low
fn check_guess(guess: i32, secret: i32) -> i32{

    if guess < secret {
        return -1;
    } else if guess > secret {
        return 1;
    } else {
        return 0;
    }

}

fn assignment3(){
    // Use a loop to repeatedly:
    // Set a mutable guess variable to a number of your choice (simulating user input)
    // Call the check_guess function
    // Use an if-else expression to print whether the guess was correct, too high, or too low
    // If the guess was correct, break the loop

    // count variable to count the number of guesses
    let mut count = 0;
    // Mutable guess variable
    let secret_num = 4;
    // Array to simulate the guesses
    let guesses = [1, 5, 8, 9, 4];

    println!("The Guessing Number Game!\n");

    // For loop to check the number of guesses
    for guess in guesses {
        println!("Your guess is: {}", guess);
        count += 1;
        if check_guess(guess, secret_num) == 0 {
            println!("\nThat is correct, the secret number is {}, and you guessed {}.\n", secret_num, guess);
            break;
        }
        else if check_guess(guess, secret_num) == 1 {
            println!("\nSorry, but your guess is too high, try again!\n");
        }
        else {
            println!("\nSorry, but your guess is too low, try again!\n");
        }
    }

    // After the loop ends, print how many guesses it took (you'll need to track this in a variable)
    println!("\nIt took you {} guesses to win the game!\n", count);
}

fn main() {
    assignment1();
    println!();
    assignment2();
    println!();
    assignment3();
}
