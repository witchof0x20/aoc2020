use color_eyre::eyre::{self, WrapErr};
use std::collections::HashMap;
use std::io::{self, BufRead};
use std::time::Instant;

// This is contained in a function because rust couldn't figure out its type bounds
fn parse_bag_contents_item(s: &str) -> eyre::Result<Option<(String, usize)>> {
    let bag = s.trim_end_matches('s').trim_end_matches(" bag");
    let mut bag = bag.split(' ');
    let count = bag
        .next()
        .map(|s| if s == "no" { "0" } else { s })
        .ok_or_else(|| eyre::eyre!("Missing bag count"))?
        .parse()
        .wrap_err("Failed to parse bag count")?;
    if count == 0 {
        Ok(None)
    } else {
        //TODO: change this once split_once is stable
        let color = bag.collect::<Vec<&str>>().join(" ");
        if color.is_empty() {
            Err(eyre::eyre!("Bag color is empty"))
        } else {
            Ok(Some((color, count)))
        }
    }
}

fn part1(rules: &HashMap<String, Option<HashMap<String, usize>>>, goal_color: &str) -> usize {
    rules
        .keys()
        .map(|color| can_contain(rules, color, goal_color))
        .map(|x| if x { 1 } else { 0 })
        .sum()
}
fn can_contain(
    rules: &HashMap<String, Option<HashMap<String, usize>>>,
    color: &str,
    other: &str,
) -> bool {
    if let Some(Some(contents)) = rules.get(color) {
        if contents.contains_key(other) {
            true
        } else {
            contents
                .keys()
                .any(|color| can_contain(rules, color, other))
        }
    } else {
        false
    }
}

fn part2(rules: &HashMap<String, Option<HashMap<String, usize>>>, goal_color: &str) -> usize {
    bag_count(rules, goal_color) - 1
}

fn bag_count(rules: &HashMap<String, Option<HashMap<String, usize>>>, goal_color: &str) -> usize {
    match rules.get(goal_color) {
        Some(Some(contents)) => {
            contents
                .iter()
                .map(|(color, count)| count * bag_count(rules, color))
                .sum::<usize>()
                + 1
        }
        Some(None) => 1,
        None => 0,
    }
}

// Problem constant
const GOAL_COLOR: &str = "shiny gold";

fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    let stdin = io::stdin();
    // Start the timer
    let start_time = Instant::now();
    // Create the map
    let rules: HashMap<String, Option<HashMap<String, usize>>> = stdin
        .lock()
        .lines()
        .map(|line| {
            line.wrap_err("Failed to read line").and_then(|line| {
                let mut parts = line.trim_end_matches('.').split(" bags contain ");
                let outer_bag = parts
                    .next()
                    .ok_or_else(|| eyre::eyre!("Missing containing bag"))?
                    .to_owned();
                let inner_bags = parts
                    .next()
                    .ok_or_else(|| eyre::eyre!("Missing contained bag"))?;
                if parts.next().is_some() {
                    return Err(eyre::eyre!("Extra info at end"));
                }
                let inner_bags: Option<HashMap<String, usize>> = inner_bags
                    .split(", ")
                    .map(parse_bag_contents_item)
                    .collect::<Result<_, _>>()?;
                Ok((outer_bag, inner_bags))
            })
        })
        .collect::<eyre::Result<_>>()?;
    // Solve part 1
    println!("Part 1: {}", part1(&rules, GOAL_COLOR));
    // Solve part 2
    println!("Part 2: {}", part2(&rules, GOAL_COLOR));
    // Stop the timer
    let time_elapsed = start_time.elapsed();
    println!("Solved in {:?}", time_elapsed);
    Ok(())
}
