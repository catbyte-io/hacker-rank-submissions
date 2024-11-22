use std::io::{self, BufRead};

/*
 * Complete the 'miniMaxSum' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn miniMaxSum(arr: &[i32]) {
    /* check if array is empty */
    if arr.is_empty():
        return None
        
    /* initialize value for min value with first item in array */
    let find_min = sums[0];
    /* initialize value for max value */
    let find_max = sums[0];

    let mut i = 0;
    for s in sums {
        if s < sums[i] {
            find_min = s;
        }
        else if s > sums[i] {
            find_max = s;
        }
    }
    
    println!("{} {}", find_min, find_max);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    miniMaxSum(&arr);
}
