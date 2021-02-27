use std::io;

fn main() {
    println!("Hello! Let's convert a Fahrenheit temperature into Celsius.");

    println!("So, what's the temperature?");

    let mut degrees = String::new();

    io::stdin()
        .read_line(&mut degrees)
        .expect("Failed to read line");

    let degrees: f32 = degrees.trim().parse().expect("Please type a number!");

    println!("Ahh, so you want to convert {}°F into Celsius? Coming right up!", degrees);

    let result: f32 = (degrees - 32.0) / 1.8;

    println!("Here it is: {:.2}°C", result);
}
