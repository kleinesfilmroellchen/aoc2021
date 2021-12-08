use num::Num;
use std::fs::read_to_string;
use std::str::FromStr;

fn main() {
    part2();
}

fn gauß<T>(n: T) -> T
where
    T: Num + Copy,
{
    (n * n + n) / (T::one() + T::one())
}

fn part2() {
    let crabs = read_to_string("input")
        .unwrap()
        .split(',')
        .map(|days| i32::from_str(days.trim()).unwrap())
        .collect::<Vec<i32>>();
    let max = *crabs.iter().max().unwrap();
    let min = *crabs.iter().min().unwrap();

    let min_dist = (min..=max)
        .map(|target| {
            crabs
                .iter()
                .map(|pos| gauß((target - pos).abs()) as u64)
                .sum::<u64>()
        })
        .inspect(|total| println!("{}", total))
        .min()
        .unwrap();

    println!("{}", min_dist);
}

fn part1() {
    let crabs = read_to_string("input")
        .unwrap()
        .split(',')
        .map(|days| i32::from_str(days.trim()).unwrap())
        .collect::<Vec<i32>>();
    let max = *crabs.iter().max().unwrap();
    let min = *crabs.iter().min().unwrap();

    let min_dist = (min..=max)
        .map(|target| {
            crabs
                .iter()
                .map(|pos| (target - pos).abs() as u64)
                .sum::<u64>()
        })
        .inspect(|total| println!("{}", total))
        .min()
        .unwrap();

    println!("{}", min_dist);
}
