use std::io::{self, BufRead};

/*
 * Complete the 'plusMinus' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn plus_minus(arr: &[i32]) {
    let mut positive_counts = 0;
    let mut negative_counts = 0;
    let mut zero_counts = 0;
    let len = arr.len();

    for i in 0..len {
        if arr[i] > 0 {
           positive_counts += 1; 
        } else if arr[i] < 0 {
            negative_counts += 1;
        } else {
            zero_counts += 1;
        }
    }

    let len = len as f64;
    let results = [
        positive_counts as f64 / len, 
        negative_counts as f64 / len, 
        zero_counts as f64 / len]
    .to_vec();

    for i in 0..results.len() {
        println!("{:.6}", results[i]);
    }
}

pub fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    plus_minus(&arr);
}
