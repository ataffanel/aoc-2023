use std::{collections::HashMap, str::FromStr};

use anyhow::{anyhow, Error};
use itertools::Itertools;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Coordinate {
    x: i32,
    y: i32,
}

struct Map {
    pipes: HashMap<Coordinate, (Coordinate, Coordinate)>,
    start: Coordinate,
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pipes: HashMap<_, _> = s
            .lines()
            .enumerate()
            .map(|(y, line)| {
                let y = y as i32;
                line.chars()
                    .enumerate()
                    .filter(|(_, tile)| *tile != '.')
                    .map(move |(x, tile)| {
                        let x = x as i32;
                        let directions = match tile {
                            '-' => (Coordinate { x: x - 1, y }, Coordinate { x: x + 1, y }),
                            '|' => (Coordinate { x, y: y - 1 }, Coordinate { x, y: y + 1 }),
                            'L' => (Coordinate { x, y: y - 1 }, Coordinate { x: x + 1, y }),
                            'J' => (Coordinate { x, y: y - 1 }, Coordinate { x: x - 1, y }),
                            '7' => (Coordinate { x: x - 1, y }, Coordinate { x, y: y + 1 }),
                            'F' => (Coordinate { x: x + 1, y }, Coordinate { x, y: y + 1 }),
                            // Start is a special case, we do not know (yet) it's direction,
                            // it points to itself for now, solve it later down
                            'S' => (Coordinate { x, y }, Coordinate { x, y }),

                            _ => return Err(anyhow!("Bad format, tile '{}' unknown", tile)),
                        };

                        Ok::<_, Error>((Coordinate { x, y }, directions))
                    })
            })
            .flatten()
            // .inspect(|e| { dbg!(e); })
            .collect::<Result<_, _>>()?;

        let start = pipes
            .iter()
            .filter(|(pos, (n1, n2))| *pos == n1 && *pos == n2)
            .map(|(pos, _)| pos)
            .next()
            .ok_or(anyhow!("No start!"))?
            .clone();

        // Resolve start directions
        let directions = pipes
            .iter()
            // .inspect(|e| {
            //     dbg!(e);
            // })
            .filter(|(pos, (n1, _))| *pos != n1) // Skip start ...
            .filter(|(_, (n1, n2))| *n1 == start || *n2 == start)
            .map(|(pos, _)| *pos)
            // .inspect(|e| {
            //     dbg!(e);
            // })
            .collect_tuple()
            .ok_or(anyhow!("Map error, 2 tiles should link to start"))?;

        pipes.insert(start, directions);

        Ok(Self { pipes, start })
    }
}

impl Map {
    fn loop_length(&self) -> Result<u32, Error> {
        let mut len = 1;
        let mut visited = HashMap::new();

        let mut current_pos = self.pipes[&self.start].0;
        visited.insert(self.start, true);

        while current_pos != self.start {
            visited.insert(current_pos, true);

            // dbg!(&current_pos);

            let (n1, n2) = self
                .pipes
                .get(&current_pos)
                .cloned()
                .ok_or(anyhow!("Map error, {:?} not found", current_pos))?;

            current_pos = if !visited.get(&n1).cloned().unwrap_or_default() || (n1 == self.start && len > 1) {
                n1
            } else if !visited.get(&n2).cloned().unwrap_or_default() || (n2 == self.start && len > 1) {
                n2
            } else {
                return Err(anyhow!("Walk error, nowhere to go ..."));
            };

            len += 1;
        }

        Ok(len)
    }

    fn print_pipes(&self) {
        let width = self.pipes.keys().map(|pos| pos.x).max().unwrap() + 1;
        let height = self.pipes.keys().map(|pos| pos.y).max().unwrap() + 1;

        print!("   ");
        for x in 0..width {
            print!("{}", x%10);
        }
        println!();

        for y in 0..height {
            print!("{:2} ", y%100);

            for x in 0..width {
                let (n1, n2) = self.pipes.get(&Coordinate { x, y }).cloned().unwrap_or((Coordinate{x,y}, Coordinate{x,y}));

                let tile = match ((n1.x-x, n1.y-y), (n2.x-x, n2.y-y)) {
                    ((-1, 0), (1, 0)) | ((1, 0), (-1, 0)) => '─',
                    ((0, 1), (0, -1)) | ((0, -1), (0, 1)) => '│',
                    ((-1, 0), (0, -1)) | ((0, -1), (-1, 0)) => '┘',
                    ((1, 0), (0, -1)) | ((0, -1), (1, 0)) => '└',
                    ((1, 0), (0, 1)) | ((0, 1), (1, 0)) => '┌',
                    ((-1, 0), (0, 1)) | ((0, 1), (-1, 0)) => '┐',

                    _ => '╳',
                };

                print!("{}", tile);
            }
            println!();
        }
    }
}

static INPUT: &str = include_str!("../input.txt");

fn main() -> Result<(), Error> {
    let map = Map::from_str(INPUT)?;
    map.print_pipes();

    println!("Part 1: Both way meet in the middle at distance {}", map.loop_length()?/2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static SIMPLE_EXAMPLE: &str = r#"-L|F7
7S-7|
L|7||
-L-J|
L|-JF"#;

    static COMPLEX_EXAMPLE: &str = r#"..F7.
.FJ|.
SJ.L7
|F--J
LJ..."#;

    #[test]
    fn test_simple_example() -> Result<(), Error> {
        let map = Map::from_str(SIMPLE_EXAMPLE)?;
        map.print_pipes();
        let expected = 4;

        let actual = map.loop_length()? / 2;

        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_complex_example() -> Result<(), Error> {
        let map = Map::from_str(COMPLEX_EXAMPLE)?;
        map.print_pipes();
        let expected = 8;

        let actual = map.loop_length()? / 2;

        assert_eq!(expected, actual);
        Ok(())
    }
}
