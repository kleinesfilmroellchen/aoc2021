#![feature(array_zip)]

use std::fs;
use std::str::FromStr;

fn main() {
    part2();
}

fn commonness() -> [i64; 12] {
    let mut linecount = 0;
    fs::read_to_string("input")
        .unwrap()
        .lines()
        .inspect(|_| linecount += 1)
        .map(|line| {
            line.chars()
                .map(|c| if c == '0' { 0u8 } else { 1u8 })
                .collect::<Vec<u8>>()
        })
        .fold([0u64; 12], |bitcounter, line| {
            bitcounter
                .zip(line[..12].try_into().unwrap())
                .map(|(old, new)| old + new as u64)
        })
        .map(|ones_count| {
            if ones_count > linecount / 2 {
                1
            } else if ones_count == linecount / 2 {
                -1
            } else {
                0
            }
        })
}

fn part1() {
    let (gamma, epsilon) = commonness()
        .as_slice()
        .iter()
        .fold((0, 0), |(gamma, epsilon), new_bit| {
            ((gamma << 1) + new_bit, (epsilon << 1) + (1 - new_bit))
        });

    println!("{} {} {}", gamma, epsilon, gamma * epsilon);
}

fn part2() {
    let c = commonness();
    let numbers = fs::read_to_string("input")
        .unwrap()
        .lines()
        .map(|line| u64::from_str_radix(line, 2).unwrap())
        .collect::<Vec<u64>>();

    let mut bit_index: i64 = 11;
    let mut oxygen = 0;
    let mut filtered_numbers = numbers.clone();
    while bit_index >= 0 && filtered_numbers.len() > 1 {
        oxygen = filtered_numbers[0];
        // dbg!(&filtered_numbers, bit_index);
        filtered_numbers = filtered_numbers
            .iter()
            // .inspect(|num| {
            //     dbg!((*num & (1u64 << bit_index)) >> bit_index);
            //     ()
            // })
            .filter(|num| {
                ((*num & (1u64 << bit_index)) >> bit_index)
                    == sanitize_bit(c[11-bit_index as usize], 1)
            })
            .copied()
            .collect();
        bit_index -= 1;
    }

    bit_index = 11;
    let mut co2 = 0;
    filtered_numbers = numbers.clone();
    while bit_index >= 0 && filtered_numbers.len() > 1 {
        co2 = filtered_numbers[0];
        filtered_numbers = filtered_numbers
            .iter()
            // .inspect(|num| println!("{}", ((*num & (1u64 << bit_index)) >> bit_index)))
            .filter(|num| {
                ((*num & (1u64 << bit_index)) >> bit_index)
                    != sanitize_bit(c[11-bit_index as usize], 0)
            })
            .copied()
            .collect();
        bit_index -= 1;
    }

    println!("{:?} {} {} {}", c, oxygen, co2, oxygen * co2);
}

fn sanitize_bit(bit: i64, stalemate_case: u64) -> u64 {
    if bit == -1 {
        stalemate_case
    } else {
        bit as u64
    }
}
