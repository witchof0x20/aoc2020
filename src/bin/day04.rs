use color_eyre::eyre::{self, WrapErr};
use std::cmp::Ordering;
use std::io::{self, BufRead};
use std::str::FromStr;
use std::time::{Duration, Instant};

#[derive(Clone, Default)]
struct PassportIncomplete {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl PassportIncomplete {
    fn complete(self) -> Option<PassportComplete> {
        match (
            self.byr, self.iyr, self.eyr, self.hgt, self.hcl, self.ecl, self.pid,
        ) {
            (Some(byr), Some(iyr), Some(eyr), Some(hgt), Some(hcl), Some(ecl), Some(pid)) => {
                Some(PassportComplete {
                    byr,
                    iyr,
                    eyr,
                    hgt,
                    hcl,
                    ecl,
                    pid,
                    cid: self.cid,
                })
            }
            _ => None,
        }
    }
}

#[derive(Debug)]
struct PassportComplete {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: Option<String>,
}

impl PassportComplete {
    fn validate(self) -> Result<Passport, eyre::Error> {
        let byr = self
            .byr
            .parse()
            .wrap_err("Failed to parse birth year as number")?;
        let byr = if (1920..=2002).contains(&byr) {
            Ok(byr)
        } else {
            Err(eyre::eyre!("Birth year invalid"))
        }?;
        let iyr = self
            .iyr
            .parse()
            .wrap_err("Failed to parse issued year as number")?;
        let iyr = if (2010..=2020).contains(&iyr) {
            Ok(iyr)
        } else {
            Err(eyre::eyre!("Issued year invalid"))
        }?;
        let eyr = self
            .eyr
            .parse()
            .wrap_err("Failed to parse expiration year as number")?;
        let eyr = if (2020..=2030).contains(&eyr) {
            Ok(eyr)
        } else {
            Err(eyre::eyre!("Expired year invalid"))
        }?;
        let hgt = self.hgt.parse()?;
        let mut hcl_it = self.hcl.chars();
        if let Some('#') = hcl_it.next() {
        } else {
            return Err(eyre::eyre!("Hair color must begin with '#'"));
        }
        for c in hcl_it {
            if !"0123456789abcdef".contains(c) {
                return Err(eyre::eyre!("Invalid character in hair color: {}", c));
            }
        }
        let hcl = self.hcl;
        let ecl = self.ecl.parse()?;
        let pid = if self.pid.len() == 9 {
            self.pid.parse().wrap_err("Failed to parse pid as integet")
        } else {
            Err(eyre::eyre!("Invalid pid length"))
        }?;
        Ok(Passport {
            byr,
            iyr,
            eyr,
            hgt,
            hcl,
            ecl,
            pid,
            cid: self.cid,
        })
    }
}

#[derive(Debug)]
enum EyeColor {
    Amb,
    Blu,
    Brn,
    Gry,
    Grn,
    Hzl,
    Oth,
}
impl FromStr for EyeColor {
    type Err = eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use EyeColor::*;
        match s {
            "amb" => Ok(Amb),
            "blu" => Ok(Blu),
            "brn" => Ok(Brn),
            "gry" => Ok(Gry),
            "grn" => Ok(Grn),
            "hzl" => Ok(Hzl),
            "oth" => Ok(Oth),
            _ => Err(eyre::eyre!("Invalid color")),
        }
    }
}

#[derive(Debug)]
enum Height {
    Inches(u8),
    Centimeters(u8),
}

impl FromStr for Height {
    type Err = eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num_len = s.len().max(2) - 2;
        // Take all but the last 2 chars
        let height_num = s.chars().take(num_len).collect::<String>();
        let height_num: u8 = height_num
            .parse()
            .wrap_err("Failed to parse height as number")?;
        let height_unit: String = s.chars().skip(num_len).collect();
        match height_unit.as_str() {
            "cm" => {
                if (150..=193).contains(&height_num) {
                    Ok(Height::Centimeters(height_num))
                } else {
                    Err(eyre::eyre!("Invalid centimeters value"))
                }
            }
            "in" => {
                if (59..=76).contains(&height_num) {
                    Ok(Height::Inches(height_num))
                } else {
                    Err(eyre::eyre!("Invalid inches value"))
                }
            }
            _ => Err(eyre::eyre!("Invalid unit")),
        }
    }
}

#[derive(Debug)]
struct Passport {
    byr: u16,
    iyr: u16,
    eyr: u16,
    hgt: Height,
    hcl: String,
    ecl: EyeColor,
    pid: u32,
    cid: Option<String>,
}

fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    let stdin = io::stdin();
    // Start the timer
    let start_time = Instant::now();
    // Read in part 1 valid passports
    let complete_passports: Vec<PassportComplete> = stdin
        .lock()
        .lines()
        // Add an extra empty line at the end
        .chain(vec![Ok("".into())])
        .scan(
            PassportIncomplete::default(),
            |mut state, line| match line {
                Ok(line) => {
                    if line.is_empty() {
                        let passport = state.clone();
                        *state = PassportIncomplete::default();
                        Some(Ok(passport.complete()))
                    } else {
                        for pair in line.split_whitespace() {
                            let mut pair = pair.split(':');
                            let key = match pair
                                .next()
                                .ok_or_else(|| eyre::eyre!("Failed to get passport key"))
                            {
                                Ok(key) => key,
                                Err(err) => return Some(Err(err)),
                            };
                            let value = match pair
                                .next()
                                .ok_or_else(|| eyre::eyre!("Failed to get passport value"))
                            {
                                Ok(value) => value.to_owned(),
                                Err(err) => return Some(Err(err)),
                            };
                            match key {
                                "byr" => state.byr = Some(value),
                                "iyr" => state.iyr = Some(value),
                                "eyr" => state.eyr = Some(value),
                                "hgt" => state.hgt = Some(value),
                                "hcl" => state.hcl = Some(value),
                                "ecl" => state.ecl = Some(value),
                                "pid" => state.pid = Some(value),
                                "cid" => state.cid = Some(value),
                                _ => {}
                            }
                        }
                        Some(Ok(None))
                    }
                }
                Err(err) => Some(Err(err).wrap_err("Failed to read line")),
            },
        )
        .filter_map(
            |passport: Result<Option<PassportComplete>, _>| match passport {
                Ok(Some(passport)) => Some(Ok(passport)),
                Ok(None) => None,
                Err(err) => Some(Err(err)),
            },
        )
        .collect::<Result<Vec<PassportComplete>, _>>()?;
    // The number of remaining passports is the answer to part 1
    println!("Part 1: {}", complete_passports.len());
    // Do part 2
    let valid_passports = complete_passports
        .into_iter()
        .filter_map(|passport| passport.validate().ok())
        .count();
    println!("Part 2: {}", valid_passports);
    // Stop the timer
    let time_elapsed = start_time.elapsed();
    println!("Solved in {:?}", time_elapsed);

    Ok(())
}
