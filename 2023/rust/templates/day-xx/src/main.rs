mod part1;
mod part2;

use common::{accept_part_arg, solve_part, Part};
use std::env::args;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let part1_input = include_str!("../inputs/part1.in.txt");
    let part2_input = include_str!("../inputs/part1.in.txt");

    let part = accept_part_arg(args())?;
    let input = match part {
        Part::Part1 => part1_input,
        Part::Part2 => part2_input,
    };
    let solver = match part {
        Part::Part1 => part1::solve,
        Part::Part2 => part2::solve,
    };
    solve_part(part, solver, input)?;

    Ok(())
}
