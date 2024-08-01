use std::io::{self, BufRead};

/*
 * Complete the 'plusMinus' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn plusMinus(arr: &[i32]) {
    /* get array length for ratio calculation */
    let n = arr.len() as f64;
    
    /* initialize counters for positive negative and zeros */
    let mut pos_num: f64 = 0.0;
    let mut neg_num: f64 = 0.0;
    let mut zeros: f64 = 0.0;
    
    /* iterate through array and decipher positive negative or zero */
    for (i, x) in arr.iter().enumerate() {
        if x.signum() == 1 {
            pos_num += 1.0;
        } 
        else if x.signum() == -1 {
            neg_num += 1.0;
        }
        else {
            zeros += 1.0;
        }
    }
    
    /* ratios */
    let pos_ratio: f64 = pos_num / n;
    let neg_ratio: f64 = neg_num / n;
    let zero_ratio: f64 = zeros / n;
    
    /* print results */
    println!("{:.6}", pos_ratio);
    println!("{:.6}", neg_ratio);
    println!("{:.6}", zero_ratio);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    plusMinus(&arr);
}
