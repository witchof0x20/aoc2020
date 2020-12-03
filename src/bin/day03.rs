use color_eyre::eyre::{self, WrapErr};
use std::cmp::Ordering;
use std::io::{self, BufRead};
use std::str::FromStr;
use std::time::{Duration, Instant};

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
