use std::str::FromStr;
use anyhow::Error;
use itertools::Itertools;
use rayon::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Coordinate {
    x: i64,
    y: i64,
}

impl Coordinate {
    fn distance(&self, other: Coordinate) -> u64{
        let mut dx = (self.x - other.x).abs() ;
        let mut dy = (self.y - other.y).abs();

        let mut step = 0;

        while dx > 0 || dy > 0 {
            step += 1;
            if dx > dy {
                dx -= 1;
            } else {
                dy -= 1;
            }
        }

        step
    }
}

struct Image {
    data: Vec<Coordinate>
}

impl FromStr for Image {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line
                    .char_indices()
                    .filter(|(_, cell)| *cell == '#')
                    .map(move |(x, _)| Coordinate{x: x as i64, y: y as i64})
            })
            .flatten()
            .collect();

        Ok(Self{data})
    }
}

impl Image {
    fn expand_universe_with(&mut self, expansion: i64) {
        let width = self.data.iter().map(|c| c.x).max().unwrap() + 1;
        let height = self.data.iter().map(|c| c.y).max().unwrap() + 1;

        for x in (0..width).rev() {
            if self.data.iter().filter(|c| c.x == x).count() == 0 {
                self.data.iter_mut().filter(|c| c.x > x).for_each(|c| c.x += expansion);
            }
        }

        for y in (0..height).rev() {
            if self.data.iter().filter(|c| c.y == y).count() == 0 {
                self.data.iter_mut().filter(|c| c.y > y).for_each(|c| c.y += expansion);
            }
        }
    }

    fn sum_distances(&self) -> u64 {
        self.data
            .iter()
            .tuple_combinations()
            .par_bridge()
            .map(|(a, b)| a.distance(*b))
            .sum()
    }
}

static INPUT: &str = include_str!("../input.txt");

fn main() -> Result<(), Error> {
    let mut image: Image = Image::from_str(INPUT)?;
    image.expand_universe_with(1);

    println!("Part 1: Sum of all the distances: {}", image.sum_distances());

    let mut image: Image = Image::from_str(INPUT)?;
    image.expand_universe_with(1000000 - 1);

    println!("Part 1: Sum of all the distances after super expansion: {}", image.sum_distances());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = include_str!("../example.txt");
    
    #[test]
    fn test_coordinate_distance() -> Result<(), Error>{
        let mut image = Image::from_str(EXAMPLE)?;
        image.expand_universe_with(1);
        let expected = 5;

        let actual = image.data[7].distance(image.data[8]);

        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn test_part1() -> Result<(), Error> {
        let mut image = Image::from_str(EXAMPLE)?;
        image.expand_universe_with(1);
        let expected = 374;

        let actual = image.sum_distances();

        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn test_part2_10() -> Result<(), Error> {
        let mut image = Image::from_str(EXAMPLE)?;
        image.expand_universe_with(9);
        let expected = 1030;

        let actual = image.sum_distances();

        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn test_part2_100() -> Result<(), Error> {
        let mut image = Image::from_str(EXAMPLE)?;
        image.expand_universe_with(99);
        let expected = 8410;

        let actual = image.sum_distances();

        assert_eq!(expected, actual);

        Ok(())
    }
}