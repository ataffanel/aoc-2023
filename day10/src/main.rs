use std::{collections::HashMap, str::FromStr, fs::Metadata, ops::{Sub, Add}};

use anyhow::{anyhow, Error};
use itertools::Itertools;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Sub<Coordinate> for Coordinate {
    type Output = Vec2d;

    fn sub(self, rhs: Coordinate) -> Self::Output {
        Vec2d{ x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl Add<Vec2d> for Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: Vec2d) -> Self::Output {
        Coordinate { x: self.x + rhs.x, y: self.y + rhs.y}
    }
}

impl Sub<Vec2d> for Coordinate {
    type Output = Coordinate;

    fn sub(self, rhs: Vec2d) -> Self::Output {
        Coordinate { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
enum Property {
    #[default]
    Unknown,
    Track,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Vec2d {
    x: i32,
    y: i32,
}

impl Vec2d {
    fn rotate_90(self) -> Self {
        Vec2d{ x: -self.y, y: self.x}
    }
}

struct Map {
    pipes: HashMap<Coordinate, (Coordinate, Coordinate)>,
    start: Coordinate,
    metadata: HashMap<Coordinate, Property>,
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

        Ok(Self { pipes, start, metadata: HashMap::new() })
    }
}

impl Map {
    fn loop_length(&mut self) -> Result<u32, Error> {
        let mut len = 1;
        let mut visited = HashMap::new();

        let mut prev_pos = self.start;
        let mut current_pos = self.pipes[&self.start].0;
        visited.insert(self.start, true);
        self.metadata.insert(self.start, Property::Track);

        while current_pos != self.start {
            visited.insert(current_pos, true);
            self.metadata.insert(current_pos, Property::Track);

            // dbg!(&current_pos);

            let (n1, n2) = self
                .pipes
                .get(&current_pos)
                .cloned()
                .ok_or(anyhow!("Map error, {:?} not found", current_pos))?;

            let next_pos = if !visited.get(&n1).cloned().unwrap_or_default() || (n1 == self.start && len > 1) {
                n1
            } else if !visited.get(&n2).cloned().unwrap_or_default() || (n2 == self.start && len > 1) {
                n2
            } else {
                return Err(anyhow!("Walk error, nowhere to go ..."));
            };

            // Calculate and set left/right properties for prev->current and current->next pos vector.
            for vec in [current_pos - prev_pos, next_pos - current_pos] {
                let left = current_pos + vec.rotate_90();
                let right = current_pos - vec.rotate_90();

                // dbg!(prev_pos, current_pos, next_pos, vec, left, right);

                if !self.metadata.contains_key(&left) {
                    self.metadata.insert(left, Property::Left);
                }
                if !self.metadata.contains_key(&right) {
                    self.metadata.insert(right, Property::Right);
                }
            }

            prev_pos = current_pos;
            current_pos = next_pos;
            len += 1;
        }

        // Finish to fill properties for all cells, expand LEFT/RIGHT cells to neighbors
        for _ in 0..100 {
            let width = self.pipes.keys().map(|pos| pos.x).max().unwrap() + 1;
            let height = self.pipes.keys().map(|pos| pos.y).max().unwrap() + 1;

            for x in 0..width {
                for y in 0..height {
                    match self.metadata.get(&Coordinate{x, y}).cloned() {
                        Some(p) if p == Property::Left || p == Property::Right => {
                            for (vx, vy) in [(-1, 0), (1, 0), (0, -1), (0, 1), (-1, -1)] {
                                let target = Coordinate{x, y} + Vec2d{ x: vx, y: vy};
                                if !self.metadata.contains_key(&target) && (0..width).contains(&target.x) && (0..height).contains(&target.y) {
                                    self.metadata.insert(target, p);
                                }
                            }
                        }
                        _ => (),
                    }
                }
            }
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
                let property = self.metadata.get(&Coordinate{x,y}).cloned().unwrap_or_default();

                let color = match property {
                    Property::Unknown => "",
                    Property::Left => "\x1B[30;32m",
                    Property::Right => "\x1B[30;31m",
                    Property::Track => "\x1B[30;34m",
                };

                let tile = match ((n1.x-x, n1.y-y), (n2.x-x, n2.y-y)) {
                    ((-1, 0), (1, 0)) | ((1, 0), (-1, 0)) => "─",
                    ((0, 1), (0, -1)) | ((0, -1), (0, 1)) => "│",
                    ((-1, 0), (0, -1)) | ((0, -1), (-1, 0)) => "┘",
                    ((1, 0), (0, -1)) | ((0, -1), (1, 0)) => "└",
                    ((1, 0), (0, 1)) | ((0, 1), (1, 0)) => "┌",
                    ((-1, 0), (0, 1)) | ((0, 1), (-1, 0)) => "┐",

                    _ => "╳",
                };

                print!("{}{}\x1B[0m", color, tile);
            }
            println!();
        }
    }
}

static INPUT: &str = include_str!("../input.txt");

fn main() -> Result<(), Error> {
    let mut map = Map::from_str(INPUT)?;

    println!("Part 1: Both way meet in the middle at distance {}", map.loop_length()?/2);

    map.print_pipes();

    println!("Part 2: Number of \x1B[30;31mRight cell: {}\x1B[0m, \x1B[30;32mLeft cell: {}\x1B[0m", map.metadata.values().filter(|v| **v == Property::Right).count(), map.metadata.values().filter(|v| **v == Property::Left).count());


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

static PART2_EXAMPLE: &str = r#"..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
.........."#;

    #[test]
    fn test_simple_example() -> Result<(), Error> {
        let mut map = Map::from_str(SIMPLE_EXAMPLE)?;
        let expected = 4;

        let actual = map.loop_length()? / 2;

        map.print_pipes();

        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_complex_example() -> Result<(), Error> {
        let mut map = Map::from_str(COMPLEX_EXAMPLE)?;
        let expected = 8;

        let actual = map.loop_length()? / 2;

        map.print_pipes();

        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2_example() -> Result<(), Error> {
        let mut map = Map::from_str(PART2_EXAMPLE)?;

        map.loop_length()?;

        map.print_pipes();

        Ok(())
    }
}
