use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum Part {
    Part1,
    Part2,
}

#[derive(Parser)]
pub struct CliArgs {
    #[arg(short, long)]
    pub part: Part,

    #[arg(short, long)]
    pub input: PathBuf,
}

pub fn read_args() -> (Part, String) {
    let args = CliArgs::parse();
    let input = std::fs::read_to_string(&args.input).expect("Failed to read input file");
    (args.part, input)
}
