use std::io::{self, BufRead};

/*
 * Complete the 'miniMaxSum' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn mini_max_sum(arr: &[i64]) {
    let mut sum = 0;
    let mut min = 0;
    let mut max = 0;

    for i in 0..arr.len() {
        sum += arr[i];
    }

    max = sum - arr[0];
    min = sum - arr[0];

    for i in 1..arr.len() {
        let temp_sum_with_except = sum - arr[i];
        
        if temp_sum_with_except > max {
            max = temp_sum_with_except;
        }

        if temp_sum_with_except < min {
            min = temp_sum_with_except;
        }
    }

    println!("{} {}", min, max);
}

pub fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let arr: Vec<i64> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i64>().unwrap())
        .collect();

    mini_max_sum(&arr);
}

