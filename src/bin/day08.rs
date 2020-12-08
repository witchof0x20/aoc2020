use color_eyre::eyre::{self, WrapErr};
use std::convert::{TryFrom, TryInto};
use std::io::{self, BufRead};
use std::str::FromStr;
use std::time::Instant;

/// Represents a single line of "assembly" code
#[derive(Debug)]
enum Instruction {
    Acc(i64),
    Jmp(isize),
    Nop(isize),
}

impl Instruction {
    /// Flips jmp and nop
    ///
    /// # Returns
    /// `true` if the instruction was flipped
    /// `false` if the instruction was not flipped
    fn flip(&mut self) -> bool {
        match self {
            Self::Acc(value) => false,
            Self::Jmp(value) => {
                *self = Self::Nop(*value);
                true
            }
            Self::Nop(value) => {
                *self = Self::Jmp(*value);
                true
            }
        }
    }
}

impl FromStr for Instruction {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split(' ');
        let op = tokens.next().ok_or_else(|| eyre::eyre!("Missing op"))?;
        match op {
            "acc" => tokens
                .next()
                .ok_or_else(|| eyre::eyre!("Missing value"))?
                .parse()
                .map(Self::Acc)
                .wrap_err("Failed to parse acc argument"),
            "jmp" => tokens
                .next()
                .ok_or_else(|| eyre::eyre!("Missing value"))?
                .parse()
                .map(Self::Jmp)
                .wrap_err("Failed to parse acc argument"),
            "nop" => tokens
                .next()
                .ok_or_else(|| eyre::eyre!("Missing value"))?
                .parse()
                .map(Self::Nop)
                .wrap_err("Failed to parse acc argument"),
            other => Err(eyre::eyre!("Invalid op: {}", other)),
        }
    }
}

/// Represents a machine that contains code
struct Machine {
    code: Vec<Instruction>,
}

/// Represents the result of running code
#[derive(Debug)]
enum RunResult {
    InfiniteLoop(i64),
    Complete(i64),
    TerminatedAbnormally(std::num::TryFromIntError),
}

impl Machine {
    /// Machine constructor
    fn new(code: Vec<Instruction>) -> Self {
        Self { code }
    }
    /// Runs the machine's code
    fn run(&self) -> RunResult {
        let mut pc = 0;
        let mut acc = 0;
        let mut visited = vec![false; self.code.len()];
        loop {
            // Try converting pc to usize
            let pc_u: usize = match pc.try_into() {
                Ok(pc_u) => pc_u,
                Err(err) => break RunResult::TerminatedAbnormally(err),
            };
            if pc_u >= self.code.len() {
                break RunResult::Complete(acc);
            }
            if visited[pc_u] {
                break RunResult::InfiniteLoop(acc);
            }
            // Mark instruction as visited
            visited[pc_u] = true;
            // Run instruction
            pc += match self.code[pc_u] {
                Instruction::Acc(value) => {
                    acc += value;
                    1
                }
                Instruction::Jmp(value) => value,
                Instruction::Nop(_) => 1,
            }
        }
    }
    /// Flips the instruction at the given index
    ///
    /// # Returns
    /// Ok(true) if instruction was flipped
    /// Ok(false) if instruction was not flipped
    /// Err(err) if index went out of bounds
    fn flip_instruction(&mut self, index: usize) -> eyre::Result<bool> {
        self.code
            .get_mut(index)
            .map(Instruction::flip)
            .ok_or_else(|| eyre::eyre!("Index {} is out of bounds", index))
    }
}

fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    let stdin = io::stdin();
    // Start the timer
    let start_time = Instant::now();
    // Create the machine
    let mut machine = stdin
        .lock()
        .lines()
        .map(|line| match line {
            Ok(line) => line.parse().wrap_err("Failed to parse opcode"),
            Err(err) => Err(err).wrap_err("Failed to read line"),
        })
        .collect::<Result<_, _>>()
        .map(Machine::new)?;
    // Part 1
    match machine.run() {
        RunResult::InfiniteLoop(value) => println!("Part 1: {}", value),
        result => {
            return Err(eyre::eyre!(
                "Part 1 finished with unexpected result: {:?}",
                result
            ))
        }
    }
    // Part 2
    // Flip each instruction
    for i in 0..machine.code.len() {
        if let Ok(true) = machine.flip_instruction(i) {
            // See if the program returns normally
            if let RunResult::Complete(value) = machine.run() {
                println!("Part 2: {}", value);
                break;
            }
            // Flip the instruction back
            machine.flip_instruction(i);
        }
    }
    // Stop the timer
    let time_elapsed = start_time.elapsed();
    println!("Solved in {:?}", time_elapsed);

    Ok(())
}
