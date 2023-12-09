#![feature(slice_group_by)]

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
struct GridNumber {
    cells: Vec<Cell>,
    x_range: (usize, usize), // (start, end)
    y: usize,
    value: usize,
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
        neighbors.sort_by(|a, b| a.y.cmp(&b.y).then(a.x.cmp(&b.x)));
        neighbors
    }

    fn get_numbers(&self) -> Vec<GridNumber> {
        let mut numbers = Vec::new();

        for (y, row) in self.cells.iter().enumerate() {
            let mut x = 0;
            while x < row.len() {
                let cell = self.get_cell(x, y).unwrap();
                if let CellContent::Digit(_) = cell.content {
                    let x_start = x;
                    let mut xd = 0;
                    let mut cells = Vec::with_capacity(self.cells[0].len());
                    let mut digits = Vec::with_capacity(self.cells[0].len());
                    while let Some(Cell {
                        content: CellContent::Digit(c),
                        x,
                        y,
                    }) = self.get_cell(x + xd, y)
                    {
                        cells.push(Cell {
                            content: CellContent::Digit(*c),
                            x: *x,
                            y: *y,
                        });
                        digits.push(*c);
                        xd += 1;
                    }
                    let value = digits.iter().fold(0, |acc, digit| acc * 10 + *digit as usize);
                    numbers.push(GridNumber {
                        cells,
                        value,
                        x_range: (x_start, x + xd),
                        y,
                    });
                    x += xd;
                }
                x += 1;
            }
        }

        numbers
    }
}

pub fn solve_part_1(input: &str) -> Result<String, Box<dyn Error>> {
    let grid = Grid::new(input);
    let all_grid_numbers = grid.get_numbers();

    let part_numbers = all_grid_numbers
        .iter()
        .filter(|num| {
            num.cells.iter().any(|c| {
                let neighbors = grid.get_neighbors(c.x, c.y);
                neighbors.iter().any(|n| matches!(n.content, CellContent::Symbol(_)))
            })
        })
        .collect::<Vec<_>>();

    let result: usize = part_numbers
        .iter()
        .map(|num| {
            num.cells.iter().filter_map(|c| match c {
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

pub fn solve_part_2(input: &str) -> Result<String, Box<dyn Error>> {
    let grid = Grid::new(input);
    let numbers = grid.get_numbers();
    let mut sum = 0;

    for (y, row) in grid.cells.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if let CellContent::Symbol('*') = cell.content {
                let mut neighboring_numbers: Vec<&GridNumber> = Vec::new();
                for Cell { content, x, y } in grid.get_neighbors(x, y) {
                    if let CellContent::Digit(_) = content {
                        let number_for_digit = numbers.iter().find(|n| n.x_range.0 <= *x && n.x_range.1 >= *x && n.y == *y);
                        if let Some(number) = number_for_digit {
                            // Check if the number is already in the list
                            if !neighboring_numbers.iter().any(|&n| n.x_range.0 == number.x_range.0 && n.y == number.y) {
                                neighboring_numbers.push(number);
                            }
                        }
                    }
                }
                if neighboring_numbers.len() != 2 {
                    continue;
                }
                let gear_ratio = neighboring_numbers.iter().fold(1, |acc, n| acc * n.value);
                sum += gear_ratio;
            }
        }
    }


    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {}
