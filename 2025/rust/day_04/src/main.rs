mod grid;

use aoc_common::{Part, read_args};

use crate::grid::{Grid, GridCell};

fn solve_part_1(input: &str) -> String {
    let grid = Grid::from_layout(input);

    let mut removable_rolls_count = 0;

    grid.iter_lines().enumerate().for_each(|(l, line)| {
        for (c, cell) in line.iter().enumerate() {
            if cell != &GridCell::Roll {
                continue;
            }
            let neighbors = grid.neighbors(l, c);
            let neighboring_rolls_count = neighbors.iter().filter(|cell| **cell == GridCell::Roll).count();
            if neighboring_rolls_count < 4 {
                removable_rolls_count += 1;
            }
        }
    });

    removable_rolls_count.to_string()
}

fn solve_part_2(input: &str) -> String {
    let mut grid = Grid::from_layout(input);

    let mut potential_removable_rolls_count = 0;

    loop {
        let mut can_remove_rolls = false;
        let mut rolls_to_remove = Vec::new();

        for l in 0..grid.lines.len() {
            for (c, cell) in grid.lines[l].iter().enumerate() {
                if cell != &GridCell::Roll {
                    continue;
                }
                let neighbors = grid.neighbors(l, c);
                let neighboring_rolls_count = neighbors.iter().filter(|cell| **cell == GridCell::Roll).count();
                if neighboring_rolls_count < 4 {
                    potential_removable_rolls_count += 1;
                    rolls_to_remove.push((l, c));
                    can_remove_rolls = true;
                }
            }
        }

        for (l, c) in rolls_to_remove {
            grid.set(l, c, GridCell::Empty);
        }

        if !can_remove_rolls {
            break;
        }
    }

    potential_removable_rolls_count.to_string()
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
        let expected = "13";
        assert_eq!(solve_part_1(input), expected);
    }

    #[test]
    fn test_part1_real() {
        let input = include_str!("../inputs/input.txt");
        let expected = "1437";
        assert_eq!(solve_part_1(input), expected);
    }

    #[test]
    fn test_solve_part_2_sample() {
        let input = include_str!("../inputs/sample.txt");
        let expected = "43";
        assert_eq!(solve_part_2(input), expected);
    }
}
