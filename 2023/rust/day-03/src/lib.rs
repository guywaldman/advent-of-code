use itertools::iproduct;
use std::error::Error;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum CellContent {
    Symbol(char),
    Digit(u8),
    Empty,
}

#[derive(Debug)]
struct Cell {
    content: CellContent,
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Grid {
    cells: Vec<Vec<Cell>>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let cells = input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        let content = match c {
                            c if c.is_ascii_digit() => CellContent::Digit(c.to_digit(10).unwrap() as u8),
                            '.' => CellContent::Empty,
                            c => CellContent::Symbol(c),
                        };
                        Cell { content, x, y }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        Self { cells }
    }

    fn get_cell(&self, x: usize, y: usize) -> Option<&Cell> {
        self.cells.get(y).and_then(|row| row.get(x))
    }

    fn get_neighbors(&self, x: usize, y: usize) -> Vec<&Cell> {
        let mut neighbors = Vec::new();
        let coords: Vec<isize> = vec![-1, 0, 1];
        for (&xd, &yd) in iproduct!(coords.iter(), coords.iter()) {
            if xd == 0 && yd == 0 {
                continue;
            }
            let (neighbor_x, neighbor_y) = ((x as isize + xd), y as isize + yd);
            if neighbor_x < 0 || neighbor_y < 0 {
                continue;
            }
            if let Some(cell) = self.get_cell(neighbor_x as usize, neighbor_y as usize) {
                neighbors.push(cell);
            }
        }
        neighbors
    }
}

pub fn solve_part_1(input: &str) -> Result<String, Box<dyn Error>> {
    let grid = Grid::new(input);
    let mut potential_part_numbers: Vec<Vec<Cell>> = Vec::new();

    for (y, row) in grid.cells.iter().enumerate() {
        let mut x = 0;
        while x < row.len() {
            let cell = grid.get_cell(x, y).unwrap();
            if let CellContent::Digit(_) = cell.content {
                let mut xd = 0;
                let mut digits = Vec::with_capacity(grid.cells[0].len());
                while let Some(Cell {
                    content: CellContent::Digit(c),
                    x,
                    y,
                }) = grid.get_cell(x + xd, y)
                {
                    digits.push(Cell {
                        content: CellContent::Digit(*c),
                        x: *x,
                        y: *y,
                    });
                    xd += 1;
                }
                potential_part_numbers.push(digits);
                x += xd;
            }
            x += 1;

            // if let CellContent::Digit(c) = cell.content {
            //     let neighbors = grid.get_neighbors(x, y);
            //     let is_adjacent_to_symbol = neighbors.iter().any(|n|
            // matches!(n.content, CellContent::Symbol(_)));
            //     if is_adjacent_to_symbol {
            //         is_in_number = false;
            //         digits.clear();
            //         x += 1;
            //         continue;
            //     }
            //     digits.push(c);
            //     is_in_number = true;
            // } else if is_in_number {
            //     // Current cell is not a digit and we were in a number, so we
            // just finished a     // number.
            //     part_numbers.push(digits.clone());
            //     digits.clear();
            //     is_in_number = false;
            // }
            // x += 1;
        }
    }

    let part_numbers = potential_part_numbers
        .iter()
        .filter(|number| {
            number.iter().any(|c| {
                let neighbors = grid.get_neighbors(c.x, c.y);
                neighbors.iter().any(|n| matches!(n.content, CellContent::Symbol(_)))
            })
        })
        .collect::<Vec<_>>();

    let result: usize = part_numbers
        .iter()
        .map(|d| {
            d.iter().filter_map(|c| match c {
                Cell {
                    content: CellContent::Digit(c),
                    ..
                } => Some(c),
                _ => None,
            })
        })
        .map(|digits| digits.fold(0, |acc, digit| acc * 10 + *digit as usize))
        .sum();

    Ok(result.to_string())
}

pub fn solve_part_2(_input: &str) -> Result<String, Box<dyn Error>> {
    todo!()
}

#[cfg(test)]
mod tests {}
