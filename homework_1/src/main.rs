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
    println!("The sum of all the numbers is {}.\n", sum);

// 5. Use a loop to find and print the largest number in the array.
    println!("\nPrint The Largest Number!");
    let mut largest_num = 0;
    for temp in array{
        if temp > largest_num{
            largest_num = temp;
        }
    }
    println!("The largest number is {}", largest_num);
}

fn main() {
    assignment1();
    println!();
    assignment2();
}
