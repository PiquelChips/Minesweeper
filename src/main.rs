#![expect(missing_docs, reason = "not finished")]

use minesweeper::{self, Grid};

fn main() {
    let grid = Grid::<5, 5>::new_random();
    grid.print_real();
    println!();
    grid.print_player();
}
