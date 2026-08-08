use anyhow::{Context, Result};
use proton::convert_inches_to_meters;

use std::io::{self, Write};

fn main() -> Result<()> {
    let mut input = String::new();

    println!("-- Convert inches to meters --");

    loop {
        print!("Enter inches (or 'q' to quit): ");
        io::stdout().flush().context("Failed to flush stdout")?;

        input.clear();
        io::stdin()
            .read_line(&mut input)
            .context("Failed to read input from stdin")?;

        let input = input.trim();
        if input.eq_ignore_ascii_case("q") {
            break;
        }

        let inches: f32 = match input.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error: '{}' is not a valid number", input);
                continue;
            }
        };

        match convert_inches_to_meters(inches) {
            Ok(meters) => {
                println!("{} inches is {} meters", inches, meters);
            }
            Err(e) => {
                eprintln!("Conversion error: {}", e);
            }
        }
    }

    Ok(())
}
