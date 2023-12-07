use std::{cmp::Ordering, str::FromStr};

use anyhow::{anyhow, Error};
use itertools::Itertools;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    C2 = 0,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    CT,
    CJ,
    CQ,
    CK,
    CA,
}

impl TryFrom<char> for Card {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '2' => Ok(Self::C2),
            '3' => Ok(Self::C3),
            '4' => Ok(Self::C4),
            '5' => Ok(Self::C5),
            '6' => Ok(Self::C6),
            '7' => Ok(Self::C7),
            '8' => Ok(Self::C8),
            '9' => Ok(Self::C9),
            'T' => Ok(Self::CT),
            'J' => Ok(Self::CJ),
            'Q' => Ok(Self::CQ),
            'K' => Ok(Self::CK),
            'A' => Ok(Self::CA),
            _ => Err(anyhow!("Unknown card: '{}'", value)),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind = 0,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl From<[Card; 5]> for HandType {
    fn from(cards: [Card; 5]) -> Self {
        let mut cards: Vec<Card> = cards.iter().cloned().collect();
        cards.sort();

        // Find similarity pattern
        let mut pattern = Vec::new();
        let mut last_c = None;
        let mut current_len = 0;
        for card in cards {
            if last_c.is_none() {
                last_c = Some(card);
            }
            if Some(card) == last_c {
                current_len += 1;
            } else {
                pattern.push(current_len);
                current_len = 1;
                last_c = Some(card);
            }
        }
        pattern.push(current_len);
        pattern.sort();
        let pattern: Vec<_> = pattern.iter().rev().collect();

        dbg!(&pattern);

        match pattern.as_slice() {
            [5, ..] => Self::FiveOfAKind,
            [4, ..] => Self::FourOfAKind,
            [3, 2, ..] => Self::FullHouse,
            [3, ..] => Self::ThreeOfAKind,
            [2, 2, ..] => Self::TwoPair,
            [2, ..] => Self::OnePair,
            _ => Self::HighCard,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Hand {
    cards: [Card; 5],
}

impl FromStr for Hand {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards = s
            .chars()
            .map(Card::try_from)
            .collect::<Result<Vec<_>, _>>()?
            .as_slice()
            .try_into()?;

        Ok(Self { cards })
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let type_comp = HandType::from(other.cards.clone()).partial_cmp(&self.cards.clone().into());
        if let Some(Ordering::Equal) = type_comp {
            self.cards.partial_cmp(&other.cards)
        } else {
            type_comp
        }
    }
}

struct Game {
    hands: Vec<(Hand, u64)>,
}

impl FromStr for Game {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hands = s
            .lines()
            .map(|line| {
                let (hand, bid) = line
                    .split(' ')
                    .next_tuple()
                    .ok_or(anyhow!("Line parsing error"))?;
                Ok::<_, Error>((Hand::from_str(hand)?, bid.parse()?))
            })
            .collect::<Result<_, _>>()?;

        Ok(Self { hands })
    }
}

impl Game {
    fn total_wining(&self) -> u64 {
        self.hands
            .iter()
            .sorted_by(|(hand, _), (other_hand, _)| hand.partial_cmp(other_hand).unwrap())
            .enumerate()
            .inspect(|v| { dbg!(v); } )
            .map(|(i, (_, bid))| (i as u64 + 1) * bid)
            .sum()
    }
}

static INPUT: &str = include_str!("../input.txt");

fn main() -> Result<(), Error> {
    let game = Game::from_str(INPUT)?;

    println!("Part 1: Game total winnings: {}", game.total_wining());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand_type_recognition() -> Result<(), Error> {
        assert_eq!(
            HandType::FiveOfAKind,
            [Card::CA, Card::CA, Card::CA, Card::CA, Card::CA].try_into()?
        );
        assert_eq!(
            HandType::FourOfAKind,
            [Card::CA, Card::CA, Card::CA, Card::C2, Card::CA].try_into()?
        );
        assert_eq!(
            HandType::FullHouse,
            [Card::CA, Card::CA, Card::C2, Card::CA, Card::C2].try_into()?
        );
        assert_eq!(
            HandType::ThreeOfAKind,
            [Card::CA, Card::C2, Card::CA, Card::C3, Card::CA].try_into()?
        );
        assert_eq!(
            HandType::TwoPair,
            [Card::CA, Card::C3, Card::C2, Card::C2, Card::C3].try_into()?
        );
        assert_eq!(
            HandType::OnePair,
            [Card::C3, Card::C2, Card::C4, Card::C3, Card::C5].try_into()?
        );
        assert_eq!(
            HandType::HighCard,
            [Card::C2, Card::C4, Card::C5, Card::C6, Card::CA].try_into()?
        );

        Ok(())
    }

    static EXAMPLE: &str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;

    #[test]
    fn test_part1_example() -> Result<(), Error> {
        let game = Game::from_str(EXAMPLE)?;
        let expected = 6440;

        let actual = game.total_wining();

        assert_eq!(expected, actual);
        Ok(())
    }
}
