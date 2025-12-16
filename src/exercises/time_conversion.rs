use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'timeConversion' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts STRING s as parameter.
 */

fn timeConversion(s: &str) -> String {
    let (time, midday) = s.split_at(s.len() - 2);

    let mut time_parts = time.splitn(3, ":");
    let mut hour = time_parts.next().unwrap().parse::<i32>().unwrap();
    let minute = time_parts.next().unwrap();
    let second = time_parts.next().unwrap();

    match midday {
        "AM" => {
            if hour == 12 {
                hour = 0;
            }
        },
        "PM" => {
            if hour < 12 {
                hour += 12;
            }
        },
        _ => unreachable!(),
    }

    if hour < 10 {
        format!("0{}:{}:{}", hour, minute, second)
    } else {
        format!("{}:{}:{}", hour, minute, second)
    }   
}

pub fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = timeConversion(&s);

    writeln!(&mut fptr, "{}", result).ok();
}

