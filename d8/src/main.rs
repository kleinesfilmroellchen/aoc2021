use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

type Segment = HashSet<char>;
// target, source
type SegmentMap = HashMap<char, char>;
type PossibleSegments = HashMap<char, HashSet<char>>;

lazy_static! {
	static ref real_segments: HashMap<u8, Segment> =
		["abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg"]
			.iter()
			.enumerate()
			.map(|(i, s)| (i as u8, parse_segment(s)))
			.collect();
	// inverted; only works for 1,4,7,8
	static ref segment_counts: HashMap<usize, u8> = real_segments
		.iter()
		.map(|(i, elts)| (elts.len(), *i))
		.collect();
}

fn parse_segment(input: &str) -> Segment {
	input.chars().collect()
}

fn main() {
	part2();
}

fn resolve_unique_segments(patterns: &Vec<Segment>) -> HashMap<u8, Segment> {
	let mut final_map = HashMap::<u8, Segment>::new();
	for pattern in patterns {
		let matching_number = segment_counts[&pattern.len()];
		if [1, 4, 7, 8].contains(&matching_number) {
			final_map.insert(matching_number, pattern.clone());
		}
	}
	final_map
}

fn part1() {
	let mut count: usize = 0;

	for line in read_to_string("input").unwrap().lines() {
		let parts = line.split(" | ").map(String::from).collect::<Vec<String>>();
		let (patterns_str, data_str) = (parts[0].clone(), parts[1].clone());
		let (patterns, datas) = (
			patterns_str
				.split(" ")
				.map(parse_segment)
				.collect::<Vec<Segment>>(),
			data_str
				.split(" ")
				.map(parse_segment)
				.collect::<Vec<Segment>>(),
		);
		let map = resolve_unique_segments(&patterns);
		count += datas
			.iter()
			.filter(|data_pattern| map.values().any(|v| v == *data_pattern))
			.count();
	}
	println!("{}", count);
}

// algorithm:
// store mapping from a target segment (a,b,c,d,e,f,g) to all the currently possible actual values in this display.
// 	as a start, that's no segments (the empty list)
// first, deduce which detected patterns belong to which numbers for the unique count numbers 1,4,7,8
// 	this means we get four mappings {x,y,z, ...} -> {u,v,w, ...}
// now, for each segment on the left sides, add all the targets that appear in at least corresponding right side
// 	and remove all the targets that appear in at least one not-corresponding right side.
// this is because the segment can (in theory) map to anything in a number that it appears in, but to nothing in a number that it doesn't appear in.
// because 'a' is a segment uniquely identified with these four, at least a now has to map to only one target.
// remove this target from all other segments, as they can no longer map to it.
// there should be another unique segment mapping now:
// 	we can repeat the above step with another unique mapping, removing its target from all other segments
// 	until all segments are uniquely mapped.
// then simply deduce each number's representation, and what `data` actually represents.
// 	the deduction can be used as a check, because all numbers should have a unique representation if we did things correctly.
//
// note: the segments we can deduce from the easily-identifyable mappings from 1,4,7,8:
// a is the segment in only 7, 8
// b, d are the segments in only 4, 8
// c, f are the segments in all 1, 4, 7, 8
// e, g are the segments in only 8

fn reduce_counts(
	possible_segments: &PossibleSegments,
	patterns: &Vec<Segment>,
) -> Option<PossibleSegments> {
	if possible_segments
		.iter()
		.all(|(_, targets)| targets.len() == 1)
	{
		let collected_segment = possible_segments
			.iter()
			.map(|(_, targets)| targets.iter().map(|t| *t).next().unwrap())
			.collect::<Segment>();
		// attention: the segments may be invalid, as multiple sources share a target or the mappings are not valid
		if collected_segment.len() == possible_segments.len()
			&& patterns.iter().all(|pattern| {
				real_segments.values().any(|val| {
					println!(
						"last check, remapped pattern {:?}, compare to {:?}",
						pattern
							.iter()
							.map(|segment| *possible_segments[segment].iter().next().unwrap())
							.collect::<Vec<char>>(),
						val
					);
					*val
						== (pattern
							.iter()
							.map(|segment| *possible_segments[segment].iter().next().unwrap())
							.collect())
				})
			}) {
			Some(possible_segments.clone())
		} else {
			// println!("<< impossible combination, returning");
			None
		}
	} else if possible_segments
		.iter()
		.any(|(_, targets)| targets.len() < 1)
	{
		None
	} else {
		// the segment with the minimum choice count
		let (min_segment_source, min_segment_targets) = possible_segments
			.iter()
			.reduce(|(source1, targets1), (source2, targets2)| {
				if targets1.len() < targets2.len() && targets1.len() != 1 {
					(source1, targets1)
				} else {
					(source2, targets2)
				}
			})
			.unwrap();
		// println!(
		// 	"-- selecting from these: {:?} {:?}",
		// 	min_segment_source, min_segment_targets
		// );

		// select all segments, remove it from all but the one where we selected it and recursively call this function
		for selected_segment in min_segment_targets.iter() {
			let new_possibilities = possible_segments
				.clone()
				.into_iter()
				.map(|(some_source, mut some_target)| {
					if some_source == *min_segment_source {
						(some_source, [*selected_segment].into_iter().collect())
					} else {
						some_target.retain(|target| target != selected_segment);
						(some_source, some_target)
					}
				})
				.collect();
			// println!(
			// 	"   with removed possibility {:?}: {:?}",
			// 	selected_segment, new_possibilities
			// );

			if let Some(result) = reduce_counts(&new_possibilities, patterns) {
				// println!("<< returning {:?}", result);
				return Some(result);
			}
		}
		None
	}
}

fn part2() {
	let mut element_sum: usize = 0;

	for line in read_to_string("input").unwrap().lines() {
		let parts = line.split(" | ").map(String::from).collect::<Vec<String>>();
		let (patterns_str, data_str) = (parts[0].clone(), parts[1].clone());
		let (patterns, datas) = (
			patterns_str
				.split(" ")
				.map(parse_segment)
				.collect::<Vec<Segment>>(),
			data_str
				.split(" ")
				.map(parse_segment)
				.collect::<Vec<Segment>>(),
		);
		let map = resolve_unique_segments(&patterns)
			.iter()
			.map(move |(number, pattern)| (real_segments[number].clone(), pattern.clone()))
			.collect::<Vec<(HashSet<char>, HashSet<char>)>>();
		println!("{:?}", map);

		// for each source, add the targets from the unique maps and then remove the targets that are impossible
		let possible_segments = ['a', 'b', 'c', 'd', 'e', 'f', 'g']
			.into_iter()
			.map(|c| (c, Segment::new()))
			.map(|(source, mut possible_targets)| {
				for (other_source, other_targets) in &map {
					if other_source.contains(&source) {
						possible_targets = possible_targets.union(&other_targets).map(|c| *c).collect();
					}
				}
				(source, possible_targets)
			})
			.map(|(source, mut possible_targets)| {
				for (other_source, other_targets) in &map {
					if !other_source.contains(&source) {
						possible_targets.retain(|target| !other_targets.contains(target))
					}
				}
				(source, possible_targets)
			})
			.collect();
		println!("SEGMENTS {:?}", possible_segments);
		let final_segments = reduce_counts(&possible_segments, &patterns)
			.unwrap()
			.into_iter()
			.map(|(source, targets)| (*targets.iter().next().unwrap(), source))
			.collect::<SegmentMap>();
		println!("{:?}", final_segments);
		let corrected_data = datas
			.iter()
			.map(|segment| {
				let remapped = segment
					.iter()
					.map(|x| final_segments[x])
					.collect::<Segment>();
				let num = real_segments
					.iter()
					.filter_map(|(num, segments)| {
						if *segments == remapped {
							println!("found match with {:?}", num);
							Some(*num)
						} else {
							None
						}
					})
					.next();
				if let Some(n) = num {
					println!("got {} for {:?}", n, remapped);
				} else {
					println!("got nothing for {:?}", remapped);
				}
				num.unwrap()
			})
			.collect::<Vec<u8>>();
		println!(
			"{:?}, {}",
			corrected_data,
			corrected_data.iter().fold(0, |x, y| (x * 10 + (*y)))
		);
		element_sum += corrected_data.iter().fold(0, |x, y| (x * 10 + (*y))) as usize;
	}
	println!("{}", element_sum);
}
