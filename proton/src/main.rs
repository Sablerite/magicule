use std::io::Write;

use std::io;

fn main() {
    let mut input = String::new();
    let conversion_value: f32 = 39.37;

    'running: loop {
        // Greeting user
        println!("-- Conver inches to meters --");
        print!("Enter inches : ");
        io::stdout().flush().expect("Unable to flush print!()");

        // Take input from user
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to take input");

        let number: f32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error that wasn't a valid number");
                break 'running;
            }
        };

        println!(
            "{} inches is {} meters converted",
            number,
            number / conversion_value
        )
    }
}
