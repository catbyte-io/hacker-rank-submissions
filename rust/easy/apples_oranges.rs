use std::io::{self, BufRead};

/*
   PASSED
 * Complete the 'countApplesAndOranges' function below.
 *
 * The function accepts following parameters:
 *  1. INTEGER s
 *  2. INTEGER t
 *  3. INTEGER a
 *  4. INTEGER b
 *  5. INTEGER_ARRAY apples
 *  6. INTEGER_ARRAY oranges
 */

fn count_apples_oranges(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) {
    // vectors to store landing positions of apples and oranges
    let mut apple_positions = Vec::new();
    let mut orange_positions = Vec::new();

    // calculate landing positions of apples
    for apple in apples {
        let position = apple + a;  // adds the landing position plus the distance of the tree
        apple_positions.push(position);
    }

    // calculate landing positions of oranges
    for orange in oranges {
        let postion = orange + b;
        orange_positions.push(postion);
    }

    // create counters for the apples and oranges that land within range
    let mut apple_count = 0;
    let mut orange_count = 0;

    // calculate apples within range
    for apple in apple_positions {
        if apple >= s && apple <= t {
            apple_count += 1;
        }
    }
    println!("{}", apple_count);

    // calculate oranges within range
    for orange in orange_positions {
        if orange >= s && orange <= t {
            orange_count += 1;
        }
    }
    println!("{}", orange_count);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let s = first_multiple_input[0].trim().parse::<i32>().unwrap();

    let t = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let second_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let a = second_multiple_input[0].trim().parse::<i32>().unwrap();

    let b = second_multiple_input[1].trim().parse::<i32>().unwrap();

    let third_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let _m = third_multiple_input[0].trim().parse::<i32>().unwrap();

    let _n = third_multiple_input[1].trim().parse::<i32>().unwrap();

    let apples: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let oranges: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    count_apples_oranges(s, t, a, b, &apples, &oranges);
}
