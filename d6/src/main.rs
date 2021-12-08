use std::fs::read_to_string;
use std::str::FromStr;

const RETIMER: u8 = 6;
const STARTTIMER: u8 = 8;

fn main() {
    part1();
}

fn part1() {
    let mut fish = read_to_string("input")
        .unwrap()
        .split(',')
        .map(|days| u8::from_str(days.trim()).unwrap())
        .collect::<Vec<u8>>();

    for i in 0..80 {
        fish = fish
            .iter()
            .flat_map(|fish| {
                if *fish == 0 {
                    // produce new fish
                    vec![RETIMER, STARTTIMER]
                } else {
                    vec![fish - 1]
                }
            })
            .collect();
    }
    println!("{}", fish.len());
}
