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


    let converted_time = time_str;
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
