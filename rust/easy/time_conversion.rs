use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};


/*
 * Complete the 'timeConversion' function below.
 * Given a time in -hour AM/PM format, convert it to military (24-hour) time.
 * The function is expected to return a STRING.
 * The function accepts STRING s as parameter.
 */

fn time_conversion(s: &str) -> String {
    // trim whitespace and convert characters to uppercase
    let time_str = s.trim().to_uppercase();

    // get the time and period parts of the string
    let (time_part, period_part) = time_str.split_at(time_str.len() - 2);

    println!("time_part: {} period_part {}", time_part, period_part);

    // get the hour
    let hour_slice = &time_part[0..2];
    let mut hour: i32 = hour_slice.parse().expect("Could not parse hour");
    println!("hour: {}", hour);

    // get the rest
    let time_slice = &time_part[2..time_part.len()];
    println!("time_slice: {}", time_slice);

    // convert period_part to String
    let period_str = String::from(period_part);

    // if the period part is PM, add 12 hours to the hour, mod 24
    if period_str == "PM" {
        if hour > 12{
            hour +=12;
        }
    }

    let converted_time = [hour.to_string(), time_slice.to_string(), period_str].join("");
    return converted_time;
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = time_conversion(&s);

    writeln!(&mut fptr, "{}", result).ok();
}
