use color_eyre::eyre::{self, WrapErr};
use std::convert::TryInto;
use std::io::{self, BufRead};
use std::time::Instant;

fn main() -> eyre::Result<()> {
    //color_eyre::install()?;
    let stdin = io::stdin();
    // Start the timer
    let start_time = Instant::now();
    // Read in part 1 valid passports
    let (part1, part2) = stdin
        .lock()
        .lines()
        // Add an extra empty line at the end
        .chain(vec![Ok("".into())])
        .scan(([false; 26], [true; 26]), |(seen, common), line| {
            match line {
                Ok(line) => {
                    if line.is_empty() {
                        let answer = Some(Ok(Some((
                            seen.iter().map(|b| if *b { 1 } else { 0 }).sum::<usize>(),
                            common.iter().map(|b| if *b { 1 } else { 0 }).sum::<usize>(),
                        ))));
                        *seen = [false; 26];
                        *common = [true; 26];
                        answer
                    } else {
                        let mut cur_line = [false; 26];
                        for c in line.chars() {
                            // Convert character to base 36 number and subtract 10 to get a 0-26 number
                            let c_num: usize = if let Some(c_num) = c.to_digit(36) {
                                match (c_num - 10).try_into() {
                                    Ok(c_num) => c_num,
                                    Err(err) => {
                                        return Some(Err(err).wrap_err(
                                            "Failed to convert character index to usize",
                                        ))
                                    }
                                }
                            } else {
                                return Some(Err(eyre::eyre!("Invalid letter")));
                            };
                            seen[c_num] = true;
                            cur_line[c_num] = true;
                        }
                        for (common, cur) in common.iter_mut().zip(&cur_line) {
                            if !cur {
                                *common = false;
                            }
                        }
                        Some(Ok(None))
                    }
                }
                Err(err) => Some(Err(err).wrap_err("Failed to read line")),
            }
        })
        .filter_map(|line| match line {
            Ok(Some(line)) => Some(Ok(line)),
            Ok(None) => None,
            Err(err) => Some(Err(err)),
        })
        .try_fold((0, 0), |(seen, common), cur_info| {
            cur_info.map(|(cur_seen, cur_common)| (seen + cur_seen, common + cur_common))
        })?;
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    // Stop the timer
    let time_elapsed = start_time.elapsed();
    println!("Solved in {:?}", time_elapsed);

    Ok(())
}
