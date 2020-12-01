use color_eyre::eyre::{self, WrapErr};
use std::cmp::Ordering;
use std::io::{self, BufRead};
use std::time::{Duration, Instant};

fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    let stdin = io::stdin();
    let mut input: Vec<u64> = stdin
        .lock()
        .lines()
        .map(|line| match line {
            Ok(line) => line
                .parse()
                .wrap_err_with(|| format!("Failed to parse line {:?} as integer", line)),
            Err(err) => Err(err).wrap_err("Failed to read line"),
        })
        .collect::<Result<Vec<_>, _>>()?;
    // Start the timer
    let start_time = Instant::now();
    // Solve the problem

    // Stop the timer
    let time_elapsed = start_time.elapsed();
    println!("Solved in {:?}", time_elapsed);

    Ok(())
}
