use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'diagonalDifference' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts 2D_INTEGER_ARRAY arr as parameter.
 */

fn diagonalDifference(arr: &[Vec<i32>]) -> i32 {
    /* find size */
    let s = arr.len();
    
    /* initiate value for left to right diagonal */
    let mut left_right: i32 = 0;
    
    /* initiate value for right to left diagonal */
    let mut right_left: i32 = 0;
    
    /* arr[0][0] + arr[1][1] + arr[2][2] */
    for i in 0..s {
        left_right += arr[i][i];
    }
    
    /* arr[0][2] + arr[1][1] + arr[2][0] */
    let mut k = s - 1;
    for j in 0..s {
        right_left += arr[j][k];
        k -= 1;
    }
    /* get the absolute difference */
    let a_difference: i32;
    
    /* check for which sum is greater */
    if left_right >= right_left {
        a_difference = left_right - right_left;
    }
    else {
        a_difference = right_left - left_right;
    }
    
    return a_difference;
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let mut arr: Vec<Vec<i32>> = Vec::with_capacity(n as usize);

    for i in 0..n as usize {
        arr.push(Vec::with_capacity(n as usize));

        arr[i] = stdin_iterator.next().unwrap().unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();
    }

    let result = diagonalDifference(&arr);

    writeln!(&mut fptr, "{}", result).ok();
}
