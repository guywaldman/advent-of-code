use anyhow::{Result, anyhow};

pub const EMPTY_CELL_MARKER: char = '.';
pub const ROLL_CELL_MARKER: char = '@';

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GridCell {
    Roll,
    Empty,
}

impl TryFrom<char> for GridCell {
    type Error = anyhow::Error;
    fn try_from(value: char) -> Result<Self> {
        match value {
            EMPTY_CELL_MARKER => Ok(Self::Empty),
            ROLL_CELL_MARKER => Ok(Self::Roll),
            c => Err(anyhow!("Could not convert {c} into a grid cell")),
        }
    }
}

#[derive(Debug)]
pub struct Grid {
    pub lines: Vec<Vec<GridCell>>,
    pub width: usize,
}

impl Grid {
    pub fn from_layout(layout: &str) -> Self {
        let lines: Vec<_> = layout
            .lines()
            .map(|line| Self::parse_line(line).unwrap_or_else(|_| panic!("Invalid line: {line}")))
            .collect();
        let width = lines[0].len();
        Self { lines, width }
    }

    pub fn iter_lines(&self) -> impl Iterator<Item = &Vec<GridCell>> {
        self.lines.iter()
    }

    pub fn set(&mut self, line: usize, col: usize, value: GridCell) {
        self.lines[line][col] = value;
    }

    pub fn neighbors(&self, line: usize, col: usize) -> Vec<GridCell> {
        let line_width = self.width;

        let col_start = (col as isize - 1isize).max(0) as usize;
        let col_end = (col + 1).min(line_width - 1);

        let top_neighbors = if line == 0 {
            vec![]
        } else {
            self.lines[line - 1][col_start..=col_end].to_vec()
        };
        let mid_neighbors: Vec<_> = self.lines[line]
            .iter()
            .enumerate()
            .map(|(i, c)| if i == col { None } else { Some(c) })
            .collect::<Vec<_>>()[col_start..=col_end]
            .iter()
            .filter_map(|c| *c)
            .copied()
            .collect();
        let bottom_neighbors = if line == line_width - 1 {
            vec![]
        } else {
            self.lines[line + 1][col_start..=col_end].to_vec()
        };
        top_neighbors
            .iter()
            .chain(mid_neighbors.iter())
            .chain(bottom_neighbors.iter())
            .copied()
            .collect()
    }

    fn parse_line(line: &str) -> Result<Vec<GridCell>> {
        let mut res = Vec::with_capacity(line.len());
        for char in line.chars() {
            res.push(char.try_into()?);
        }
        Ok(res)
    }
}
