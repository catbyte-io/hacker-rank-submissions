use std::io::{self, BufRead};

/*
 * PASSED 
 Complete the 'miniMaxSum' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn mini_max_sum(arr: &[i64]) {
    /* check if array is correct length */
    if arr.len() != 5 {
        return;
    }

    if arr.iter().any(|&x| x < 1) || arr.iter().any(|&x| x > 10_i64.pow(9)) {
        return;
    }

    // calculate sum of all array elements
    let total_sum: i64 = arr.iter().sum();
        
    // find minumum and maximum sum values
    let find_min = total_sum - arr.iter().max().unwrap(); // exclude the max element
  
    let find_max = total_sum - arr.iter().min().unwrap();  // exclude the minumum element

    
    println!("{} {}", find_min, find_max);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let arr: Vec<i64> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .filter_map(|s| s.parse::<i64>().ok())
        .collect();

    mini_max_sum(&arr);
}
