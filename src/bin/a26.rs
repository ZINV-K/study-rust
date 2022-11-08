// Topic: External crates
//
// Requirements:
// * Display the current date and time
//
// Notes:
// * Use the `chrono` crate to work with time
// * (OPTIONAL) Read the documentation section `Formatting and Parsing`
//   for examples on how to create custom time formats

use chrono::prelude::*;
use humantime::format_duration;
use std::time::Duration;

fn main() {
    let utc: DateTime<Utc> = Utc::now();
    let local: DateTime<Local> = Local::now();

    println!(
        "Local time: {:?}",
        local.format("%Y-%m-%d %H:%M:%S").to_string()
    );
    println!(
        "UTC time: {:?}",
        utc.format("%Y-%m-%d %H:%M:%S").to_string()
    );
}
