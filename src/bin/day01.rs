use color_eyre::eyre::{self, WrapErr};
use std::cmp::Ordering;
use std::io::{self, BufRead};
use std::time::{Instant};
const TARGET: u64 = 2020;

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
    // Sort the input
    input.sort_unstable();
    // Keep track of whether we've solved a part yet
    let mut part1_done = false;
    let mut part2_done = false;
    // Track an upper bound in the inner loop
    let mut inner_loop_upper_bound: usize = usize::MAX;
    // Iterate over the input
    for (i, n) in input.iter().enumerate() {
        // n1 == (TARGET-n) means n+n1=TARGET
        let p1_target = TARGET - n;
        // If we've already solved part 2, do part 1 the fast way
        match (part1_done, part2_done) {
            // If both are solved we're done
            (true, true) => break,
            // If we've already solved part 2 but not part 1,
            // do part 1 the fast way
            (false, true) => {
                if let Ok(index) = input[..i].binary_search(&p1_target) {
                    println!(
                        "Part 1: {} found after {:?}",
                        n * input[index],
                        start_time.elapsed()
                    );
                    // We can now kill the program because we already know part 2 is done, and we
                    // just solved part 2
                    break;
                }
            }
            // If we haven't solved part 2, iterate over input in an O(n^2) fashion and maybe
            // check for part 1 while we're at it
            (_, false) => {
                // Keep track of another inner loop bound, this time for our binary search
                // TODO: can i move this up a loop?
                let mut binary_search_upper_bound: usize = usize::MAX;
                for (j, n1) in input[..i.min(inner_loop_upper_bound)].iter().enumerate() {
                    // If our number is less than the p1 target, try to solve part 2
                    match n1.cmp(&p1_target) {
                        Ordering::Less => {
                            // n2 == ((TARGET-n)-n1) means n+n1+n2==target
                            let p2_target = p1_target - n1;
                            // Search for p2_target in the remaining slice
                            match input[..j.min(binary_search_upper_bound)]
                                .binary_search(&p2_target)
                            {
                                // This means the value was found
                                Ok(index) => {
                                    println!(
                                        "Part 2: {} found after {:?}",
                                        n * n1 * input[index],
                                        start_time.elapsed()
                                    );
                                    part2_done = true;
                                }
                                // This means the value was not found, but we know the index of the
                                // last value less than our target value
                                // input[index+1] > ((TARGET - n) - n1)
                                // which means
                                // input[index+1] + n + n1 > TARGET
                                // which means
                                // input[index+1] + n + n1_future > TARGET
                                // for all n1_future > n1
                                Err(index) => {
                                    binary_search_upper_bound = index + 1;
                                }
                            }
                        }
                        // If our number is equal, try to solve part 2 (it might include zero) then
                        // part 1
                        Ordering::Equal => {
                            // Part 2 can only be satisfied here if there is a zero. And a zero
                            // must exist as the first element of a sorted, unsigned array.
                            // We also can't count duplicates, so make sure our second number isn't
                            // also at the zero index
                            if j != 0 && input[0] == 0 {
                                // The answer is zero because 0 is multiplied in there
                                println!("Part 2: 0 found after {:?}", start_time.elapsed());
                                part2_done = true;
                            }
                            if !part1_done {
                                // We know we found p1 so mark it complete
                                println!(
                                    "Part 1: {} found after {:?}",
                                    n * n1,
                                    start_time.elapsed()
                                );
                                part1_done = true;
                            }
                        }
                        // if n1 > TARGET-n, then n + n1 > TARGET so break the inner loop
                        Ordering::Greater => {
                            // Store the index where this break happened
                            // If n + n1 > TARGET, then n_future + n1 > TARGET if all n_future > n
                            inner_loop_upper_bound = j;
                            break;
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
