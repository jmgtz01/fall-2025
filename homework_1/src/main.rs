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
    let mut fahrenheit: f64 = FAHREN;
    println!("Fahrenheit: {}°F -> Celcius: {:.2}°C", fahrenheit,fahrenheit_to_celsius(fahrenheit));

    for i in 1..=5{
        let f = fahrenheit + i as f64;
        println!("Fahrenheit: {}°F -> Celcius: {:.2}°C", f,fahrenheit_to_celsius(f));
    }
}

fn main() {
    assignment1();
}
