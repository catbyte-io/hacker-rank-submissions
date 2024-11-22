use std::io::{self, BufRead};

/*
 * Complete the 'staircase' function below.
 *
 * The function accepts INTEGER n as parameter.
 */

fn staircase(n: i32) {
     let mut m = n;
     loop {
        loop {
        if m <= 0 || m > 100 {
            break;
        }
        for i in 1..m {
            print!(" ");
        }
        for j in 0..(n-m) + 1 {
            print!("#");
        }
        println!("");
        m -= 1;
        }
     }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    staircase(n);
}
