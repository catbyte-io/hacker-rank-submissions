use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'migratoryBirds' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn migratory_birds(arr: &[i32]) -> i32 {
    let mut bird_counts = vec![0; 6];
    for &bird in arr.iter() {
        bird_counts[bird as usize] += 1;
    }
    let highest_count = bird_counts.iter().max().unwrap();
    // Get the first index where the highest count occurs, as this will be the lowest ID for the bird seen most
    let bird_type = bird_counts.iter().position(|&x| x == *highest_count).unwrap();
    return bird_type as i32;
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _arr_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = migratory_birds(&arr);

    writeln!(&mut fptr, "{}", result).ok();
}
