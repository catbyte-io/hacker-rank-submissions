use std::collections::HashMap;
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
    // Create dictionary to store month values
    let mut months = HashMap::new();

    // Add the month mappings up to September except February.
    months.extend([(1, 31), (3, 31), (4, 30), (5, 31), (6, 30), (7, 31), (8, 31), (9, 30)]);

    // Configure February based on year
    if year < 1918 {  // Checks for Julian calendar
        if year % 4 == 0 {  // Checks for Julian calendar leap year
            months.extend([(2, 29)]);
        }
        else {
            months.extend([(2, 28)]);
        }
    }
    else if year > 1918 {  // Checks for Gregorian calendar
        if year % 400 == 0 || (year % 4 == 0 && year % 100 != 0) {  // Checks for leap year
            months.extend([(2, 29)]);
        }
        else {
            months.extend([(2, 28)]);
        }
    }
    else { // If year is 1918
        months.extend([(2, 15)]);
    }

    // Add the month days while the sum is less than 256 
    let mut month = 1;
    let mut days = 0;
    let mut day = 0;
    while days < 256 {
        days += months[&month];
        if days >= 256 {
            days -= months[&month];
            day = 256 - days;
            break; 
        }
        month += 1;
    }

    let date = format!("{:02}.{:02}.{}", day, month, year);
    return date;
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let year = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let result = day_of_programmer(year);

    writeln!(&mut fptr, "{}", result).ok();
}
