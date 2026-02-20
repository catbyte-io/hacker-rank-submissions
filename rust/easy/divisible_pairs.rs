use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'divisibleSumPairs' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER n
 *  2. INTEGER k
 *  3. INTEGER_ARRAY ar
 */

fn divisible_sum_pairs(k: i32, ar: &[i32]) -> i32 {
    // Track the pairs
    let mut count = 0;

    for (i, &a) in ar.iter().enumerate() {
        for &b in &ar[i + 1..] {
            if (a + b) % k == 0 {
                count += 1;
            }
        }
    }
    return count;
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

    let k = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let ar: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = divisible_sum_pairs(k, &ar);

    writeln!(&mut fptr, "{}", result).ok();
}
