use std::fs::read_to_string;
use std::str::FromStr;

const RETIMER: usize = 6;
const STARTTIMER: usize = 8;

fn main() {
    part2();
}

fn part2() {
    let fish = read_to_string("input")
        .unwrap()
        .split(',')
        .map(|days| usize::from_str(days.trim()).unwrap())
        .collect::<Vec<usize>>();
    let mut fish_count = [0u128; STARTTIMER + 1];
    for f in fish {
        fish_count[f] += 1;
    }
    let mut new_fish_count = [0u128; STARTTIMER + 1];
    for i in 0..256 {
        println!("{:?}", fish_count);
        for (i, count) in fish_count.iter().enumerate() {
            if i > 0 {
                new_fish_count[i - 1] += *count;
            } else {
                new_fish_count[RETIMER] += *count;
                new_fish_count[STARTTIMER] += *count;
            }
        }
        fish_count = new_fish_count;
        new_fish_count = [0u128; STARTTIMER + 1];
    }
    println!("{}", fish_count.iter().sum::<u128>());
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
                    vec![RETIMER as u8, STARTTIMER as u8]
                } else {
                    vec![fish - 1]
                }
            })
            .collect();
    }
    println!("{}", fish.len());
}
