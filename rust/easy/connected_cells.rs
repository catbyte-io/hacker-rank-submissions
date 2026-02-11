use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'connectedCell' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts 2D_INTEGER_ARRAY matrix as parameter.
 */

fn connected_cell(matrix: &[Vec<i32>]) -> i32 {
    // determine matrix size
    let rows = matrix.len();
    let columns = matrix[0].len();

    for i in 0..rows {

    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let m = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let mut matrix: Vec<Vec<i32>> = Vec::with_capacity(n as usize);

    for i in 0..n as usize {
        matrix.push(Vec::with_capacity(m as usize));

        matrix[i] = stdin_iterator.next().unwrap().unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();
    }

    let result = connected_cell(&matrix);

    writeln!(&mut fptr, "{}", result).ok();
}
