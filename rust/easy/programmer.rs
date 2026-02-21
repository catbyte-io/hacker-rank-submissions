use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'dayOfProgrammer' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts INTEGER year as parameter.
 */

fn day_of_programmer(year: i32) -> String {
    let mut february = 0;
    if year < 1918 {  // Checks for Julian calendar
        if year % 4 == 0 { // Checks for Julian calendar leap year
            february = 29;
        }
        else {
            february = 28;
        }
    }
    else if year > 1918 { // Checks for Gregorian calendar
        if year % 400 == 0 || (year % 4 == 0 && year % 100 != 0) { // Checks for leap year
            february = 29;
        }
        else {
            february = 28;
        }
    }
    else {
        february = 14;
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let year = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let result = day_of_programmer(year);

    writeln!(&mut fptr, "{}", result).ok();
}
