use anyhow::Result;
use std::ops::RangeInclusive;

use aoc_common::{Part, read_args};

#[derive(Debug)]
struct Inventory {
    fresh_ranges: Vec<RangeInclusive<usize>>,
}

fn solve_part_1(input: &str) -> String {
    let (inventory, ids) = parse_input(input).expect("Failed to parse input");
    let fresh_count = ids
        .iter()
        .filter(|id| inventory.fresh_ranges.iter().any(|range| range.contains(id)))
        .count();
    fresh_count.to_string()
}

fn solve_part_2(input: &str) -> String {
    let (inventory, _) = parse_input(input).expect("Failed to parse input");
    let mut sorted_ranges = inventory.fresh_ranges.clone();
    sorted_ranges.sort_by_key(|range| *range.start());

    let mut merged_ranges: Vec<RangeInclusive<usize>> = Vec::new();
    for range in sorted_ranges {
        if let Some(last) = merged_ranges.last_mut() {
            if *range.start() <= last.end() + 1 {
                // Overlapping ranges
                *last = *last.start()..=*range.end().max(last.end());
            } else {
                merged_ranges.push(range);
            }
        } else {
            merged_ranges.push(range);
        }
    }

    let total_fresh = merged_ranges.iter().map(|range| range.end() - range.start() + 1).sum::<usize>();
    total_fresh.to_string()
}

fn parse_input(input: &str) -> Result<(Inventory, Vec<usize>)> {
    let parts: Vec<_> = input.split("\n\n").collect();
    let (ranges_str, ids_str) = (parts[0], parts[1]);

    let mut ranges = Vec::new();
    for range_str in ranges_str.split("\n") {
        let parts: Vec<_> = range_str.split("-").collect();
        let start: usize = parts[0].parse()?;
        let end: usize = parts[1].parse()?;
        ranges.push(start..=end);
    }

    let ids: Vec<usize> = ids_str.split("\n").map(|s| s.parse().unwrap_or(0)).collect();
    Ok((Inventory { fresh_ranges: ranges }, ids))
}

fn main() {
    let (part, input) = read_args();
    match part {
        Part::Part1 => println!("{}", solve_part_1(&input)),
        Part::Part2 => println!("{}", solve_part_2(&input)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_sample() {
        let input = include_str!("../inputs/sample.txt");
        let expected = "3";
        assert_eq!(solve_part_1(input), expected);
    }

    #[test]
    fn test_part1_real() {
        let input = include_str!("../inputs/input.txt");
        let expected = "643";
        assert_eq!(solve_part_1(input), expected);
    }

    #[test]
    fn test_solve_part_2_sample() {
        let input = include_str!("../inputs/sample.txt");
        let expected = "14";
        assert_eq!(solve_part_2(input), expected);
    }

    #[test]
    fn test_solve_part_2_real() {
        let input = include_str!("../inputs/input.txt");
        let expected = "342018167474526";
        assert_eq!(solve_part_2(input), expected);
    }
}
