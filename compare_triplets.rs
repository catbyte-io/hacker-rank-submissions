use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Challenge from https://www.hackerrank.com/challenges/compare-the-triplets
 * Complete the 'compareTriplets' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts following parameters:
 *  1. INTEGER_ARRAY a
 *  2. INTEGER_ARRAY b
 */

fn compareTriplets(a: &[i32], b: &[i32]) -> Vec<i32> {
    /* declare an array to store scores */
    let mut scores: Vec<i32> = Vec::new();
    /* initialize scores */
    let mut a_score = 0;
    let mut b_score = 0;
    /* iterate over a */
    for (i, x) in a.iter().enumerate() {
        /* compare x (a[i]) to b[i] */
        if x > &b[i]{
            a_score += 1;
        }
        else if x < &b[i] {
            b_score += 1;
        }
    }
    scores.push(a_score);
    scores.push(b_score);
    return scores;
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let a: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let b: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = compareTriplets(&a, &b);

    for i in 0..result.len() {
        write!(&mut fptr, "{}", result[i]).ok();

        if i != result.len() - 1 {
            write!(&mut fptr, " ").ok();
        }
    }

    writeln!(&mut fptr).ok();
}
