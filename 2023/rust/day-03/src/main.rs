use day03::{solve_part_1, solve_part_2};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let part = args
        .get(1)
        .expect("Please provide a part number")
        .parse::<u8>()
        .expect("Please provide a valid part number");
    let part1_puzzle_input = include_str!("../inputs/part1_puzzle.txt");
    let part2_puzzle_input = include_str!("../inputs/part2_puzzle.txt");
    let solution = match part {
        1 => solve_part_1(part1_puzzle_input).expect("Failed to solve part 1"),
        2 => solve_part_2(part2_puzzle_input).expect("Failed to solve part 2"),
        _ => panic!("Please provide a valid part number"),
    };
    println!("Part {} solution:", part);
    println!("{}", solution);
}

#[allow(dead_code)]
#[cfg(test)]
mod day03_tests {
    use super::*;
    const PART1_TEST_SOLUTION: &str = "4361";
    const PART1_PUZZLE_SOLUTION: &str = "557705";
    const PART2_TEST_SOLUTION: &str = "467835";
    const PART2_PUZZLE_SOLUTION: &str = "84266818";

    #[test]
    fn part_1_test_input() {
        let test_input = include_str!("../inputs/part1_test.txt");
        assert_eq!(solve_part_1(test_input).unwrap(), PART1_TEST_SOLUTION);
    }

    #[test]
    fn part_1_puzzle_input() {
        let puzzle_input = include_str!("../inputs/part1_puzzle.txt");
        assert_eq!(solve_part_1(puzzle_input).unwrap(), PART1_PUZZLE_SOLUTION);
    }

    #[test]
    fn part_2_test_input() {
        let test_input = include_str!("../inputs/part2_test.txt");
        assert_eq!(solve_part_2(test_input).unwrap(), PART2_TEST_SOLUTION);
    }

    #[test]
    fn part_2_puzzle_input() {
        let puzzle_input = include_str!("../inputs/part2_puzzle.txt");
        assert_eq!(solve_part_2(puzzle_input).unwrap(), PART2_PUZZLE_SOLUTION); }
}
