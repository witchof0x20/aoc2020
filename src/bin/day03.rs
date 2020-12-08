// Copyright 2020 witchof0x20
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public
// License along with this program. If not, see <https://www.gnu.org/licenses/>.
use color_eyre::eyre::{self, WrapErr};

use std::io::{self, BufRead};

use std::time::Instant;

fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    let stdin = io::stdin();
    // Start the timer
    let start_time = Instant::now();
    // Solve the problem
    let grid = stdin
        .lock()
        .lines()
        .map(|line| match line {
            Ok(line) => line
                .chars()
                .map(|c| match c {
                    '.' => Ok(false),
                    '#' => Ok(true),
                    _ => Err(eyre::eyre!("Invalid character")),
                })
                .collect::<Result<Vec<bool>, _>>(),
            Err(err) => Err(err).wrap_err("Failed to read line"),
        })
        .collect::<Result<Vec<Vec<bool>>, _>>()?;
    // List of slopes in part 2
    const PART2_SLOPES: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    // Index in the part 2 slopes list that references the part 1 slope
    const PART1_INDEX: usize = 1;
    let mut part2 = 1;
    for (i, (x_slope, y_slope)) in PART2_SLOPES.iter().enumerate() {
        let mut trees = 0;
        let mut x = 0;
        for row in grid.iter().step_by(*y_slope) {
            if row[x % row.len()] {
                trees += 1;
            }
            x += x_slope;
        }
        if i == PART1_INDEX {
            println!("Part 1: {}", trees);
        }
        part2 *= trees;
    }
    println!("Part 2: {}", part2);
    // Stop the timer
    let time_elapsed = start_time.elapsed();
    println!("Solved in {:?}", time_elapsed);

    Ok(())
}
