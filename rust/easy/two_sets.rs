use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::convert::TryInto;

/*
 * Complete the 'getTotalX' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER_ARRAY a
 *  2. INTEGER_ARRAY b
 */

fn get_total_x(a: &[i32], b: &[i32]) -> i32 {
    // Create vector to save the integers that meet the criteria
    let mut between = Vec::new();
    
    // Counter for integers
    let mut integer = a[a.len() - 1];

    // Iterate through both arrays
    while integer <= b[0] {
        let mut count = 0; // Tracks the leftovers from mod division
        for &i in a {
            count += integer % i;
        }
        for &i in b {
            count += i % integer;
        }
        // If there are no leftovers, the conditions are met
        if count == 0 {
            between.push(integer);
        }
        integer += 1;
    }
    return between.len().try_into().unwrap();  // Return the number of integers between the sets
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let _n = first_multiple_input[0].trim().parse::<i32>().unwrap();

    let _m = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let brr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let total = get_total_x(&arr, &brr);

    writeln!(&mut fptr, "{}", total).ok();
}
