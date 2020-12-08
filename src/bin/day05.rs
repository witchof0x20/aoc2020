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
use std::convert::TryInto;
use std::io::{self, BufRead};
use std::time::Instant;

fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    let stdin = io::stdin();
    // Start the timer
    let start_time = Instant::now();
    // Solve the problem
    let (min, max, mut map) =
        stdin
            .lock()
            .lines()
            .try_fold(
                (0u16, 0u16, [0u8; 128]),
                |(min, max, mut map), line| match line {
                    Ok(line) => {
                        // Row is just binary, F=0, B=1
                        let row: u8 = line.chars().take(7).try_fold(0, |acc, c| match c {
                            'F' => Ok(acc * 2),
                            'B' => Ok(acc * 2 + 1),
                            c => Err(eyre::eyre!("Invalid character: {}", c)),
                        })?;
                        // Col is also just binary
                        // R=1 L=0
                        let col: u8 =
                            line.chars().skip(7).take(3).try_fold(0, |acc, c| match c {
                                'L' => Ok(acc * 2),
                                'R' => Ok(acc * 2 + 1),
                                c => Err(eyre::eyre!("Invalid character: {}", c)),
                            })?;
                        // Set the bit
                        let mask = 1 << col;
                        map[usize::from(row)] = (map[usize::from(row)] & !mask) | mask;
                        // Get the id
                        let id = u16::from(row) * 8 + u16::from(col);
                        // Update minimum and maximum
                        Ok((min.min(id), max.max(id), map))
                    }
                    Err(err) => Err(err).wrap_err("Failed to read line"),
                },
            )?;
    println!("Part 1: {}", max);
    // Set the ends of the map to 255
    // This "fills" seats at the beginning and end because we can assume the seats aren't ours
    map[usize::from(min / 8)] |= !((1 << (min % 8)) - 1);
    map[usize::from(max / 8)] = 0b11111111;
    // Search for our seat
    let part2_row = map[usize::from(min / 8)..=usize::from(max / 8)]
        .iter()
        .position(|b| *b != 0b11111111)
        .ok_or_else(|| eyre::eyre!("Failed to find our seat"))?;
    let part2_col: usize = map[part2_row]
        .trailing_ones()
        .try_into()
        .wrap_err("Failed to convert col to usize somehow")?;
    println!("Part 2: {}", part2_row * 8 + part2_col);
    // Stop the timer
    let time_elapsed = start_time.elapsed();
    println!("Solved in {:?}", time_elapsed);
    Ok(())
}
