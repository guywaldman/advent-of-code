use std::{env::Args, error::Error};

pub enum Part {
    Part1,
    Part2,
}

pub fn accept_part_arg(mut args: Args) -> Result<Part, Box<dyn Error>> {
    let part_name = args
        .nth(1)
        .expect("Please specify which part to run (1 or 2)");
    let part_name = part_name.trim();
    if part_name != "1" && part_name != "2" {
        return Err(format!("Unknown part: {}", part_name).into());
    }

    match part_name {
        "1" => Ok(Part::Part1),
        "2" => Ok(Part::Part2),
        _ => Err(format!("Unknown part: {}", part_name).into()),
    }
}

pub fn solve_part(
    part: Part,
    solver: fn(&str) -> Result<String, Box<dyn Error>>,
    input: &str,
) -> Result<String, Box<dyn Error>> {
    let part_number = match part {
        Part::Part1 => 1,
        Part::Part2 => 2,
    };

    println!("Running part {}...", part_number);

    let solution = solver(input)?;

    println!("Solution:\n-------\n");
    println!("{}", solution);

    Ok(solution)
}
