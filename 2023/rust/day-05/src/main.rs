use day_05::solve_part_1;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let part = args.get(1).expect("Please provide a part number").parse::<u8>().expect("Please provide a valid part number");
    let part1_puzzle_input = include_str!("../inputs/part1_puzzle.txt");
    // let part2_puzzle_input = include_str!("../inputs/part2_puzzle.txt");
    let solution = match part {
        1 => solve_part_1(part1_puzzle_input).expect("Failed to solve part 1"),
        // 2 => part2(),
        _ => panic!("Please provide a valid part number")
    };
    println!("Part {} solution:", part);
    println!("{}", solution);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test_input() {
        let test_input = include_str!("../inputs/part1_test.txt");
        assert_eq!(solve_part_1(test_input).unwrap(), "6440");
    }

    #[test]
    fn part_1_puzzle_input() {
        let puzzle_input = include_str!("../inputs/part1_puzzle.txt");
        assert_eq!(solve_part_1(puzzle_input).unwrap(), "241344943");
    }
}