use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::str::FromStr;

lazy_static! {
	static ref line_regex: Regex = Regex::new(r"([0-9]+),([0-9]+) \-> ([0-9]+),([0-9]+)").unwrap();
}

#[derive(Debug, Copy, Clone)]
struct Line {
	start: (i16, i16),
	end: (i16, i16),
}

impl<'a> Line {
	fn new(source: &str) -> Line {
		let captures = line_regex.captures(source).unwrap();
		Line {
			start: (
				i16::from_str(&captures[1]).unwrap(),
				i16::from_str(&captures[2]).unwrap(),
			),
			end: (
				i16::from_str(&captures[3]).unwrap(),
				i16::from_str(&captures[4]).unwrap(),
			),
		}
	}

	fn is_straight(&self) -> bool {
		self.is_horizontal() || self.is_vertical()
	}
	fn is_horizontal(&self) -> bool {
		self.start.1 == self.end.1
	}
	fn is_vertical(&self) -> bool {
		self.start.0 == self.end.0
	}
	fn is_diagonal(&self) -> bool {
		!self.is_straight()
	}

	fn straight_points(&'a self) -> HashSet<(i16, i16)> {
		if !self.is_straight() {
			panic!("cannot call straight_points on non-straight line");
		}
		// just one y value
		if self.is_horizontal() {
			(if self.start.0 > self.end.0 {
				self.end.0..=self.start.0
			} else {
				self.start.0..=self.end.0
			})
			.map(move |x| (x, self.start.1))
			.collect()
		// just one x value
		} else if self.is_vertical() {
			(if self.start.1 > self.end.1 {
				self.end.1..=self.start.1
			} else {
				self.start.1..=self.end.1
			})
			.map(move |y| (self.start.0, y))
			.collect()
		} else {
			unreachable!()
		}
	}

	fn diagonal_points(&self) -> HashSet<(i16, i16)> {
		if !self.is_diagonal() {
			panic!("cannot call diagonal_points on non-diagonal line");
		}
		let xdir = if self.start.0 <= self.end.0 { 1 } else { -1 };
		let ydir = if self.start.1 <= self.end.1 { 1 } else { -1 };

		(0..=(self.start.0 - self.end.0).abs())
			.map(|step| (self.start.0 + xdir * step, self.start.1 + ydir * step))
			// .inspect(|point| println!("{:?} of {:?}", &point, &self))
			.collect()
	}

	fn intersection_straight(&self, other: &Line) -> Vec<(i16, i16)> {
		let other_points = other.straight_points();
		let mut int = Vec::<(i16, i16)>::new();
		for point in self.straight_points() {
			if other_points.contains(&point) {
				int.push(point);
			}
		}
		int
	}

	fn intersection(&self, other: &Line) -> Vec<(i16, i16)> {
		let self_points = if self.is_straight() {
			self.straight_points()
		} else {
			self.diagonal_points()
		};
		let other_points = if other.is_straight() {
			other.straight_points()
		} else {
			other.diagonal_points()
		};

		self_points
			.iter()
			.filter_map(|point| {
				if other_points.contains(&point) {
					Some(*point)
				} else {
					None
				}
			})
			.collect()
	}
}

fn main() {
	part1();
}

fn part1() {
	let lines = read_to_string("input")
		.unwrap()
		.lines()
		.map(|line| Line::new(line))
		// .filter(|line| line.is_straight())
		.collect::<Vec<Line>>();
	// dbg!(&lines);
	let mut intersections = HashSet::<(i16, i16)>::new();
	for i in 0..lines.len() {
		// println!("starting on {:?}", lines[i]);
		for j in i + 1..lines.len() {
			// use intersection_straight for part 1
			for coord in lines[i].intersection(&lines[j]) {
				intersections.insert(coord);
			}
		}
	}
	println!("{}", intersections.len());
}
