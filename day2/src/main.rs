use anyhow::{anyhow, Error};
use std::str::FromStr;

#[derive(Debug)]
struct RevealSet {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl FromStr for RevealSet {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let reveals = s
            .split(", ")
            .map(|r| r.split(" "))
            // .inspect(|e| {dbg!(e);})
            .map(|mut r| (r.next().unwrap().parse::<u32>().unwrap(), r.next().unwrap()));

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for (number, color) in reveals {
            match color {
                "red" => red = number,
                "green" => green = number,
                "blue" => blue = number,
                _ => return Err(anyhow!("Parsing error, bad color: '{}'", color)),
            }
        }
        Ok(Self { red, green, blue })
    }
}

impl RevealSet {
    fn evaluate(&self, red: u32, green: u32, blue: u32) -> bool {
        (self.red) <= red && (self.green <= green) && (self.blue <= blue)
    }
}

struct Game {
    reveals: Vec<RevealSet>,
}

impl FromStr for Game {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let reveals: Result<Vec<RevealSet>, Error> = s
            .split("; ")
            .map(|reveal| RevealSet::from_str(reveal))
            .collect();

        Ok(Self { reveals: reveals? })
    }
}

impl Game {
    fn evaluate(&self, red: u32, green: u32, blue: u32) -> bool {
        self.reveals.iter().fold(true, |current, reveal| {
            current && reveal.evaluate(red, green, blue)
        })
    }

    fn power(&self) -> u32 {
        let max_red = self.reveals.iter().map(|r| r.red).max().unwrap_or(0);
        let max_green = self.reveals.iter().map(|r| r.green).max().unwrap_or(0);
        let max_blue = self.reveals.iter().map(|r| r.blue).max().unwrap_or(0);

        max_red * max_green * max_blue
    }
}

struct Games {
    games: Vec<Game>,
}

impl FromStr for Games {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let games: Result<Vec<Game>, Error> = s
            .lines()
            .map(|line| line.split(": ").last().unwrap())
            .map(|game| Game::from_str(game))
            .collect();
        Ok(Self { games: games? })
    }
}

impl Games {
    fn evaluate(&self, red: u32, green: u32, blue: u32) -> u32 {
        self.games
            .iter()
            .enumerate()
            .filter(|(_, g)| g.evaluate(red, green, blue))
            .map(|(i, _)| (i + 1) as u32)
            .sum()
    }

    fn power(&self) -> u32 {
        self.games.iter().map(|g| g.power()).sum()
    }
}

static INPUT: &str = include_str!("../input.txt");

fn main() -> anyhow::Result<()> {
    let games = Games::from_str(INPUT)?;
    println!(
        "Part 1, Sum of possible games: {}",
        games.evaluate(12, 13, 14)
    );
    println!("Part 2, Sum of the games powers: {}", games.power());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1_example() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
        let expected = 8;

        let games = Games::from_str(input).unwrap();

        let actual = games.evaluate(12, 13, 14);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2_example() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
        let expected = 2286;

        let games = Games::from_str(input).unwrap();

        let actual = games.power();

        assert_eq!(actual, expected);
    }
}
