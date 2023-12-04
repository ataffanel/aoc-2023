use std::str::FromStr;

use anyhow::{anyhow, Error, Result};

struct Card {
    _id: u32,
    winning: Vec<u32>,
    have: Vec<u32>,
}

impl FromStr for Card {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (header, body) = {
            let mut split = s.split(": ");
            (
                split.next().ok_or(anyhow!("Bad format"))?,
                split.next().ok_or(anyhow!("Bad format"))?,
            )
        };

        let id = header
            .split(' ')
            .last()
            .ok_or(anyhow!("Bad header format"))?
            .parse::<u32>()?;
        let mut split_body = body.split(" | ");
        let winning = split_body
            .next()
            .ok_or(anyhow!("Bad body format"))?
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|n| n.parse())
            .collect::<Result<_, _>>()?;
        let have = split_body
            .next()
            .ok_or(anyhow!("Bad body format"))?
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|n| n.parse())
            .collect::<Result<_, _>>()?;

        Ok(Self {
            _id: id,
            winning,
            have,
        })
    }
}

impl Card {
    fn score(&self) -> u32 {
        let mut score = 0;
        for number in &self.have {
            if self.winning.contains(number) {
                if score == 0 {
                    score = 1;
                } else {
                    score *= 2;
                }
            }
        }
        score
    }

    fn match_count(&self) -> usize {
        self.have
            .iter()
            .filter(|n| self.winning.contains(n))
            .count()
    }
}

struct Cards {
    cards: Vec<Card>,
}

impl FromStr for Cards {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards = s
            .lines()
            .map(Card::from_str)
            .collect::<Result<_, _>>()?;

        Ok(Self { cards })
    }
}

impl Cards {
    fn score(&self) -> u32 {
        self.cards.iter().map(|card| card.score()).sum()
    }

    fn process(&self) -> u32 {
        let mut instances = vec![1; self.cards.len()];

        for (i, card) in self.cards.iter().enumerate() {
            let n_matches = card.match_count();
            let n_instance = instances[i];
            if n_matches > 0 {
                for instance in &mut instances[(i + 1)..=(i + n_matches)] {
                    *instance += n_instance;
                }
            }
        }

        instances.iter().sum()
    }
}

static INPUT: &str = include_str!("../input.txt");

fn main() -> Result<()> {
    let cards = Cards::from_str(INPUT)?;

    println!("Part 1: Sum of the cards score: {}", cards.score());
    println!("Part 2: The total number of card is {}", cards.process());

    Ok(())
}

#[cfg(test)]
mod tests {
    static EXAMPLE: &str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

    use anyhow::Result;

    use super::*;
    #[test]
    fn test_part1_example() -> Result<()> {
        let cards = Cards::from_str(EXAMPLE)?;

        let expected_score = 13;

        let actual_score = cards.score();

        assert_eq!(expected_score, actual_score);

        Ok(())
    }

    #[test]
    fn test_part2_example() -> Result<()> {
        let cards = Cards::from_str(EXAMPLE)?;

        let expected = 30;

        let actual = cards.process();

        assert_eq!(expected, actual);

        Ok(())
    }
}
