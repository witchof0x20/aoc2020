use color_eyre::eyre::{self, WrapErr};
use std::cmp::Ordering;
use std::io::{self, BufRead};
use std::str::FromStr;
use std::time::{Duration, Instant};

#[derive(Debug)]
struct PolicyPassword {
    a: usize,
    b: usize,
    letter: char,
    password: String,
}

impl FromStr for PolicyPassword {
    type Err = eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split(' ');
        let a_b = tokens
            .next()
            .ok_or_else(|| eyre::eyre!("range/indices are missing"))?;
        let mut a_b = a_b.split('-');
        let a = a_b
            .next()
            .ok_or_else(|| eyre::eyre!("Failed to get range minimum / first index"))?
            .parse()
            .wrap_err("Failed to parse range minimum / first index")?;
        let b = a_b
            .next()
            .ok_or_else(|| eyre::eyre!("Failed to get range maximum / first index"))?
            .parse()
            .wrap_err("Failed to parse range maximum / first index")?;
        let letter = tokens
            .next()
            .ok_or_else(|| eyre::eyre!("Letter is missing"))?
            .chars()
            .next()
            .ok_or_else(|| eyre::eyre!("Letter is empty"))?;
        let password = tokens
            .next()
            .ok_or_else(|| eyre::eyre!("password is missing"))?
            .to_owned();
        if tokens.next().is_some() {
            return Err(eyre::eyre!("Extra tokens in line"));
        }
        Ok(Self {
            a,
            b,
            letter,
            password,
        })
    }
}

impl PolicyPassword {
    fn valid_part1(&self) -> bool {
        (self.a..=self.b).contains(
            &self
                .password
                .chars()
                .filter(|c| *c == self.letter)
                .take(self.b + 1)
                .count(),
        )
    }
    fn valid_part2(&self) -> bool {
        let mut chars = self.password.chars();
        let c_a = chars
            .nth(self.a - 1)
            .map(|c| c == self.letter)
            .unwrap_or(false);
        let c_b = chars
            .nth((self.b - self.a) - 1)
            .map(|c| c == self.letter)
            .unwrap_or(false);
        c_a ^ c_b
    }
}

fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    let stdin = io::stdin();
    // Start the timer
    let start_time = Instant::now();
    // Solve the problem
    let (part1, part2) = stdin
        .lock()
        .lines()
        .map(|line| match line {
            Ok(line) => line
                .parse()
                .wrap_err_with(|| format!("Failed to parse line {:?} as policy/password", line)),
            Err(err) => Err(err).wrap_err("Failed to read line"),
        })
        .try_fold(
            (0, 0),
            |(valid_1, valid_2), policy_pass: Result<PolicyPassword, _>| {
                policy_pass.map(|policy_pass| {
                    (
                        valid_1 + if policy_pass.valid_part1() { 1 } else { 0 },
                        valid_2 + if policy_pass.valid_part2() { 1 } else { 0 },
                    )
                })
            },
        )?;
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    // Stop the timer
    let time_elapsed = start_time.elapsed();
    println!("Solved in {:?}", time_elapsed);

    Ok(())
}
