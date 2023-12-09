use anyhow::{anyhow, Error};
use itertools::Itertools;
use lcmx::lcmx;
use std::{collections::HashMap, str::FromStr};

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            _ => Err(anyhow!("Invalid direction char: '{}'", value)),
        }
    }
}

#[derive(Debug)]
struct Map {
    instructions: Vec<Direction>,
    network: HashMap<String, (String, String)>,
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (instructions, network): (&str, &str) = s
            .split("\n\n")
            .take(2)
            .collect_tuple()
            .ok_or(anyhow!("format error1"))?;

        let instructions = instructions
            .chars()
            .map(Direction::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        let network = network
            .lines()
            .map(|line| {
                let (location, next): (&str, &str) = line
                    .split(" = ")
                    .collect_tuple()
                    .ok_or(anyhow!("format error2"))?;

                let location = location.to_string();

                let (left, right): (String, String) = next[1..next.len() - 1]
                    .split(", ")
                    .map(str::to_string)
                    .collect_tuple()
                    .ok_or(anyhow!("format error3"))?;

                Ok::<_, Error>((location, (left, right)))
            })
            .collect::<Result<_, _>>()?;

        Ok(Self {
            instructions,
            network,
        })
    }
}

impl Map {
    fn steps(&self) -> u32 {
        let mut step = 0;
        let mut location = "AAA".to_string();
        for direction in self.instructions.iter().cycle() {
            step += 1;
            location = match direction {
                &Direction::Left => self.network[&location].0.clone(),
                &Direction::Right => self.network[&location].1.clone(),
            };

            if location == "ZZZ" {
                break;
            }
        }

        step
    }

    fn parallel_steps(&self) -> u64 {
        let tracks = self
            .network
            .keys()
            .filter(|location| location.ends_with('A'));

        let lengths = tracks
            .map(|start| {
                let mut step = 0;
                let mut location = start.to_string();
                for direction in self.instructions.iter().cycle() {
                    step += 1;
                    location = match direction {
                        &Direction::Left => self.network[&location].0.clone(),
                        &Direction::Right => self.network[&location].1.clone(),
                    };

                    if location.ends_with('Z') {
                        break;
                    }
                }

                step as u64
            })
            .collect_vec();

        lcmx(&lengths).unwrap()
    }
}

static INPUT: &str = include_str!("../input.txt");

fn main() -> Result<(), Error> {
    let map = Map::from_str(INPUT)?;

    println!("Part 1: Steps to exit: {}", map.steps());
    println!(
        "Part 2: Steps to exit parallel tracks: {}",
        map.parallel_steps()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() -> Result<(), Error> {
        let map = Map::from_str(
            r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#,
        )?;
        let expected = 2;

        let actual = map.steps();

        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn test_example_2() -> Result<(), Error> {
        let map = Map::from_str(
            r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#,
        )?;
        let expected = 6;

        let actual = map.steps();

        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn test_parallel_steps() -> Result<(), Error> {
        let map = Map::from_str(
            r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#,
        )?;
        let expected = 6;

        let actual = map.parallel_steps();

        assert_eq!(expected, actual);

        Ok(())
    }
}
