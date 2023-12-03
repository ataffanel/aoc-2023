use anyhow::{Error, Result};
use std::{collections::HashMap, ops::Range, str::FromStr};

#[derive(Debug)]
struct Schematic {
    numbers: HashMap<(i32, i32), u32>,
    symbols: HashMap<(i32, i32), char>,
}

impl FromStr for Schematic {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut numbers = HashMap::new();
        let mut symbols = HashMap::new();
        for (y, mut line) in s.lines().enumerate() {
            let y = y as i32;
            let mut x: i32 = 0;

            loop {
                match line.chars().next() {
                    Some('.') => {
                        line = &line[1..];
                        x += 1;
                    }
                    Some(c) if c.is_ascii_digit() => {
                        let len = line
                            .find(|c: char| !c.is_ascii_digit())
                            .unwrap_or(line.len());
                        numbers.insert((x, y), line[0..len].parse::<u32>()?);
                        x += len as i32;
                        line = &line[len..];
                    }
                    Some(c) => {
                        symbols.insert((x, y), c);
                        line = &line[1..];
                        x += 1;
                    }
                    _ => break,
                }
            }
        }

        Ok(Self { numbers, symbols })
    }
}

impl Schematic {
    fn find_symbol(&self, x_range: Range<i32>, y_range: Range<i32>) -> Option<char> {
        for x in x_range {
            for y in y_range.clone() {
                if let Some(c) = self.symbols.get(&(x, y)) {
                    return Some(*c);
                }
            }
        }
        None
    }

    fn number_range(x: i32, y: i32, number: u32) -> (Range<i32>, Range<i32>) {
        let len = (number.checked_ilog10().unwrap_or(0) + 1) as i32;
        let x_range = (x - 1)..(x + len + 1);
        let y_range = (y - 1)..(y + 2);
        (x_range, y_range)
    }

    fn sum_numbers(&self) -> u32 {
        self.numbers
            .iter()
            .filter(|(position, number)| {
                let (x_range, y_range) = Self::number_range(position.0, position.1, **number);
                self.find_symbol(x_range, y_range).is_some()
            })
            .map(|(_, number)| number)
            .sum()
    }

    fn gear_ratio(&self, x: i32, y: i32) -> u64 {
        let numbers: Vec<u64> = self
            .numbers
            .iter()
            .filter(|(position, number)| {
                let (x_range, y_range) = Self::number_range(position.0, position.1, **number);
                x_range.contains(&x) && y_range.contains(&y)
            })
            .map(|(_, number)| *number as u64)
            .collect();

        if numbers.len() > 1 {
            numbers.iter().product()
        } else {
            0
        }
    }

    fn sum_gear_ratios(&self) -> u64 {
        self.symbols
            .iter()
            .filter(|(_, s)| **s == '*')
            .map(|((x, y), _)| self.gear_ratio(*x, *y))
            .sum()
    }
}

static INPUT: &str = include_str!("../input.txt");

fn main() -> Result<()> {
    let schematic = Schematic::from_str(INPUT)?;
    println!(
        "Part 1: Sum of the good numbers: {}",
        schematic.sum_numbers()
    );
    println!(
        "Part 2: Sum of the gear ratios: {}",
        schematic.sum_gear_ratios()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1_example() {
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
        let schematic = Schematic::from_str(input).unwrap();

        let expected = 4361;

        let actual = schematic.sum_numbers();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2_example() {
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
        let schematic = Schematic::from_str(input).unwrap();

        let expected = 467835;

        let actual = schematic.sum_gear_ratios();

        assert_eq!(actual, expected);
    }
}
