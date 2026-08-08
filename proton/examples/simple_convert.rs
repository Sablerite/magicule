//! Simple example demonstrating the proton library usage.
//!
//! This example shows how to use the proton library to convert inches to meters.

fn main() {
    // Example values to convert
    let inches_values = [0.0, 1.0, 12.0, 39.3701, 72.0];

    println!("Inches to Meters Conversion Examples");
    println!("====================================");

    for &inches in &inches_values {
        match proton::convert_inches_to_meters(inches) {
            Ok(meters) => {
                println!("{:>8} inches = {:>8.4} meters", inches, meters);
            }
            Err(e) => {
                eprintln!("Error converting {} inches: {}", inches, e);
            }
        }
    }
}
