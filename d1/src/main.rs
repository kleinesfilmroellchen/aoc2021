#![feature(destructuring_assignment)]

use std::fs;

fn main() {
    let mut prev_sum: i64 = -1;
    let mut a: i64 = -1;
    let mut b: i64 = -1;
    let mut c: i64 = -1;
    let mut depth_increases = 0;
    for depth_str in fs::read_to_string("input").unwrap().lines() {
        let depth: i64 = depth_str.trim().parse().unwrap();
        (a, b, c) = (depth, a, b);
        if a != -1 && b != -1 && c != -1 {
            let sum = a + b + c;
            if sum > prev_sum && prev_sum != -1 {
                depth_increases += 1;
            }
            prev_sum = sum;
        }
    }
    println!("{}", depth_increases);
}
