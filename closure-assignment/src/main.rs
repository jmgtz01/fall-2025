fn track_changes() {
    let mut tracker = 0;
    let mut update = || {
        // Your implementation here
        tracker += 1;
        println!("Tracker count: {}", tracker);
    };

    update();
    update();
}

fn main() {
    let operation = |a: i32, b: i32| {
        // Your implementation here
        a * b
    };

    println!("Result: {}", operation(10, 5));

    track_changes();
}
