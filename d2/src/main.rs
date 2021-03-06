use lazy_static::*;
use regex::Regex;
use std::fs;
use std::str::FromStr;

lazy_static! {
    static ref forward: Regex = Regex::new(r"forward ([0-9]+)").unwrap();
    static ref down_or_up: Regex = Regex::new(r"(down|up) ([0-9]+)").unwrap();
}

fn main() {
    part2();
}

fn part1() {
    let (depth, distance): (i64, i64) = fs::read_to_string("input")
        .unwrap()
        .lines()
        .filter_map(|line| {
            if let Some(mtch) = down_or_up.captures(line) {
                Some((
                    i64::from_str(&mtch[2]).unwrap()
                        * match &mtch[1] {
                            "down" => 1,
                            "up" => -1,
                            _ => unreachable!(),
                        },
                    0,
                ))
            } else if let Some(mtch) = forward.captures(line) {
                Some((0, i64::from_str(&mtch[1]).unwrap()))
            } else {
                None
            }
        })
        .fold((0, 0), |(a, b), (c, d)| (a + c, b + d));
    println!(
        "depth {}, distance {}, mult {}",
        depth,
        distance,
        depth * distance
    );
}

fn part2() {
    let mut aim = 0;
    let mut distance = 0;
    let mut depth = 0;
    for line in fs::read_to_string("input").unwrap().lines() {
        if let Some(mtch) = down_or_up.captures(line) {
            aim += i64::from_str(&mtch[2]).unwrap()
                * match &mtch[1] {
                    "down" => 1,
                    "up" => -1,
                    _ => unreachable!(),
                };
        } else if let Some(mtch) = forward.captures(line) {
            let val = i64::from_str(&mtch[1]).unwrap();
            depth += aim * val;
            distance += val;
        }
    }
    println!(
        "depth {}, distance {}, mult {}",
        depth,
        distance,
        depth * distance
    );
}
