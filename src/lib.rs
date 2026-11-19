#![feature(vec_from_fn)]
#![expect(unused, reason = "not finished")]
#![expect(missing_docs, reason = "not finished")]

use std::fmt::Display;

#[derive(Debug)]
pub enum Cell {
    Mine,
    MineFlag,
    SafeFlag,
    SafeUntouched,
    Mined,
}

impl Cell {
    fn new_random() -> Self {
        if rand::random_bool(0.5) {
            Self::Mine
        } else {
            Self::SafeUntouched
        }
    }

    fn is_mine(&self) -> bool {
        matches!(self, Self::Mine | Self::MineFlag)
    }

    fn is_flag(&self) -> bool {
        matches!(self, Self::SafeFlag | Self::MineFlag)
    }

    /// Returns if this Cell is winnig. ie, if it is either a Mine or has been Mined
    fn winning(&self) -> bool {
        !matches!(self, Self::SafeUntouched | Self::SafeFlag)
    }
}

#[derive(Debug)]
pub struct Grid<const W: usize, const H: usize>([[Cell; W]; H]);

impl<const W: usize, const H: usize> Grid<W, H> {
    pub fn new_random() -> Self {
        Self(std::array::from_fn(|_| {
            std::array::from_fn(|_| Cell::new_random())
        }))
    }

    fn count_mines(&self) -> usize {
        self.0
            .iter()
            .map(|row| row.iter().filter(|cell| cell.is_mine()).count())
            .sum()
    }

    /// Returns if all safe cells have been Mined.
    fn has_won(&self) -> bool {
        self.0.iter().any(|row| row.iter().any(Cell::winning))
    }

    fn count_adjacent(&self, pos: Position<W, H>) -> usize {
        pos.neighbours()
            .filter(|pos| self.cell(pos).is_mine())
            .count()
    }

    fn cell(&self, &Position(x, y): &Position<W, H>) -> &Cell {
        &self.0[y][x]
    }

    pub fn print_player(&self) {
        self.0.iter().enumerate().for_each(|(y, row)| {
            row.iter().enumerate().for_each(|(x, cell)| {
                let char = match cell {
                    Cell::MineFlag | Cell::SafeFlag => "F",
                    Cell::Mine | Cell::SafeUntouched => "X",
                    Cell::Mined => match self.count_adjacent(Position(x, y)) {
                        0 => " ",
                        count => &format!("{count}"),
                    },
                };
                print!("{}", char);
            });
            println!();
        });
    }

    pub fn print_real(&self) {
        self.0.iter().for_each(|row| {
            row.iter().for_each(|cell| {
                let char = match cell {
                    Cell::MineFlag => "M",
                    Cell::SafeFlag => "F",
                    Cell::SafeUntouched => "X",
                    Cell::Mine => "m",
                    Cell::Mined => " ",
                };
                print!("{}", char);
            });
            println!();
        })
    }
}

/// An `(x, y)` coordinate guaranteed to be inside a `W` x `H` grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position<const W: usize, const H: usize>(usize, usize);

impl<const W: usize, const H: usize> Position<W, H> {
    /// Returns every in-bounds position surrounding this one (up to 8).
    fn neighbours(self) -> impl Iterator<Item = Self> {
        let Self(x, y) = self;
        (-1..=1)
            .flat_map(|dy| (-1..=1).map(move |dx| (dx, dy)))
            .filter(|&offset| offset != (0, 0))
            .filter_map(move |(dx, dy)| {
                Self::new(x.checked_add_signed(dx)?, y.checked_add_signed(dy)?).ok()
            })
    }

    fn new(x: usize, y: usize) -> Result<Self, PositionError> {
        if x >= W {
            return Err(PositionError::XOutOfBounds { x, width: W });
        }
        if y >= H {
            return Err(PositionError::YOutOfBounds { y, height: H });
        }
        Ok(Self(x, y))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PositionError {
    #[error("x coordinate {x} is out of bounds for grid width {width}")]
    XOutOfBounds { x: usize, width: usize },
    #[error("y coordinate {y} is out of bounds for grid height {height}")]
    YOutOfBounds { y: usize, height: usize },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neighbours_stay_in_bounds() {
        let count = |x, y| Position::<3, 3>::new(x, y).unwrap().neighbours().count();
        assert_eq!(count(0, 0), 3);
        assert_eq!(count(1, 0), 5);
        assert_eq!(count(1, 1), 8);
        assert_eq!(count(2, 2), 3);
    }

    #[test]
    fn count_adjacent_counts_mines() {
        use Cell::*;
        let grid = Grid([
            [Mine, Mined, Mine],
            [Mined, Mined, MineFlag],
            [Mined, Mined, Mined],
        ]);
        assert_eq!(grid.count_adjacent(Position(1, 1)), 3);
        assert_eq!(grid.count_adjacent(Position(0, 2)), 0);
        assert_eq!(grid.count_mines(), 3);
        assert!(grid.has_won());
    }
}
