use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'breakingRecords' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts INTEGER_ARRAY scores as parameter.
 */

fn breaking_records(scores: &[i32]) -> Vec<i32> {
    let mut records = Vec::new();

    // Track highest and lowest scores
    let mut highest_record = scores[0];
    let mut lowest_record = scores[0];

    // Trackers for record breaks
    let mut highest_record_breaks = 0;
    let mut lowest_record_breaks = 0;

    for &score in scores {
        if score > highest_record {
            highest_record = score;
            highest_record_breaks += 1;
        }
        if score < lowest_record {
            lowest_record = score;
            lowest_record_breaks += 1;
        }
    }
    records.push(highest_record_breaks);
    records.push(lowest_record_breaks);
    return records;
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let scores: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = breaking_records(&scores);

    for i in 0..result.len() {
        write!(&mut fptr, "{}", result[i]).ok();

        if i != result.len() - 1 {
            write!(&mut fptr, " ").ok();
        }
    }

    writeln!(&mut fptr).ok();
}
