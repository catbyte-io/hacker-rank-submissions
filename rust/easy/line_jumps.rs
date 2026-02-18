use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
* PASSING
 * Complete the 'kangaroo' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts following parameters:
 *  1. INTEGER x1
 *  2. INTEGER v1
 *  3. INTEGER x2
 *  4. INTEGER v2
 */

fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> String {
    let max_jumps = 10000;
    // Check if the velocity is slower and the starting point behind for the first kangaroo
    if x2 > x1 && v2 > v1 {
        return "NO".to_string();  // The kangaroo will never catch up
    }
    // Check other kangaroo
    if x1 > x2 && v1 > v2 {
        return "NO".to_string();
    }

    for i in 0..max_jumps {
        let k1 = v1 * i + x1;
        let k2 = v2 * i + x2;
        if k1 == k2 {
            return "YES".to_string();
        }
    }
    return "NO".to_string();
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let x1 = first_multiple_input[0].trim().parse::<i32>().unwrap();

    let v1 = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let x2 = first_multiple_input[2].trim().parse::<i32>().unwrap();

    let v2 = first_multiple_input[3].trim().parse::<i32>().unwrap();

    let result = kangaroo(x1, v1, x2, v2);

    writeln!(&mut fptr, "{}", result).ok();
}
