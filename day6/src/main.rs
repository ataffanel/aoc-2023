use std::{collections::HashMap, str::FromStr};

use anyhow::{Error, anyhow};

struct Races {
    races: HashMap<u64, u64>,
}

impl FromStr for Races {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let times = lines
            .next()
            .ok_or(anyhow!("not enough lines"))?
            .split(' ')
            .skip(1)
            .filter(|s| !s.is_empty())
            .map(u64::from_str)
            .collect::<Result<Vec<_>,_>>()?;
        let distances = lines
            .next()
            .ok_or(anyhow!("not enough lines"))?
            .split(' ')
            .skip(1)
            .filter(|s| !s.is_empty())
            .map(u64::from_str)
            .collect::<Result<Vec<_>,_>>()?;

        if times.len() != distances.len() {
            return Err(anyhow!("Number of time and distance not matching"));
        }

        let races = times
            .iter()
            .enumerate()
            .map(|(i, t)| (*t, distances[i]))
            .collect();

        Ok(Races { races })
    }
}

impl Races {
    fn way_to_win(&self) -> u64 {
        self
            .races
            .iter()
            .map(|(time, best)| {
                (1..time-1)
                    .map(|push| push * (*time - push))
                    .filter(|distance| distance > best)
                    .count() as u64
            })
            .product()
    }
}

static INPUT: &str = r#"Time:        40     81     77     72
Distance:   219   1012   1365   1089"#;

static INPUT2: &str = r#"Time:        40817772
Distance:   219101213651089"#;

fn main() -> Result<(), Error>{
    let races = Races::from_str(INPUT)?;

    println!("Part 1: Ways to win: {}", races.way_to_win());
    
    let races = Races::from_str(INPUT2)?;
    println!("Part 2: Ways to win THE race: {}", races.way_to_win());

    Ok(())
}


#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::*;

    static EXAMPLE: &str=r#"Time:      7  15   30
Distance:  9  40  200"#;

    #[test]
    fn test_part1() -> Result<()>{
        let races = Races::from_str(EXAMPLE)?;
        let expected = 288;

        let actual = races.way_to_win();

        assert_eq!(actual, expected);

        Ok(())
    }
}