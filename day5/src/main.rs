use anyhow::{anyhow, Error};
use std::ops::Range;
use std::{collections::HashMap, str::FromStr};
use itertools::Itertools;
use rayon::iter::{ParallelIterator, ParallelBridge};

static INPUT: &str = include_str!("../input.txt");

#[derive(Debug)]
struct Map {
    source: String,
    destination: String,
    map: HashMap<Range<u64>, Range<u64>>,
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let mut header = lines
            .next()
            .ok_or(anyhow!("Empty map"))?
            .trim_end_matches(" map:")
            .split("-to-");
        let source = header
            .next()
            .ok_or(anyhow!("Map header parsing error"))?
            .to_owned();
        let destination = header
            .next()
            .ok_or(anyhow!("Map header parsing error"))?
            .to_owned();

        let mut map = HashMap::new();
        for line in lines {
            let content: Vec<u64> = line
                .split(' ')
                .map(u64::from_str)
                .collect::<Result<_, _>>()?;
            if content.len() != 3 {
                return Err(anyhow!("Wrong map line"));
            }
            let source_range = content[1]..(content[1] + content[2]);
            let destination_range = content[0]..(content[0] + content[2]);
            map.insert(source_range, destination_range);
        }

        Ok(Self {
            source,
            destination,
            map,
        })
    }
}

impl Map {
    fn source(&self) -> String {
        self.source.clone()
    }

    fn destination(&self) -> String {
        self.destination.clone()
    }

    fn map(&self, source: u64) -> u64 {
        let mut range_finder = self.map.iter().filter(|(sr, _)| sr.contains(&source));

        if let Some((source_range, destination_range)) = range_finder.next() {
            // dbg!(source_range, destination_range);
            destination_range.start + (source - source_range.start)
        } else {
            source
        }
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    maps: HashMap<String, Map>,
}

impl FromStr for Almanac {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sections = s.split("\n\n");

        let seeds_section = sections.next().ok_or(anyhow!("Bad format, no seeds"))?;
        let seeds = seeds_section
            .split(": ")
            .last()
            .ok_or(anyhow!("Bad format"))?
            .split(' ')
            .map(u64::from_str)
            .collect::<Result<_, _>>()?;

        let maps = sections
            .map(|section| {
                let map = Map::from_str(section)?;
                Ok::<(String, Map), Error>((map.source(), map))
            })
            .collect::<Result<_, _>>()?;

        Ok(Self { seeds, maps })
    }
}

impl Almanac {
    fn walk_map(
        &self,
        source: impl AsRef<str>,
        desination: impl AsRef<str>,
        mut value: u64,
    ) -> Result<u64, Error> {
        let mut current_map = source.as_ref().to_string();
        while current_map != desination.as_ref() {
            // dbg!(&current_map, value);
            let map = self
                .maps
                .get(&current_map)
                .ok_or(anyhow!("Map {} does not exists", current_map))?;
            value = map.map(value);
            current_map = map.destination();
        }

        Ok(value)
    }

    fn find_smallest_distance(&self) -> Result<u64, Error> {
        let mut locations: Vec<u64> = self
            .seeds
            .iter()
            .map(|s| self.walk_map("seed", "location", *s))
            .collect::<Result<_, _>>()?;
        locations.sort();
        // dbg!(&locations);
        Ok(locations[0])
    }

    fn find_smallest_range_distance(&self) -> Result<u64, Error> {
        let seed_range = self.seeds.iter().tuples().map(|(start, len)| *start..(*start+*len));

        let location = seed_range
            .par_bridge()
            .map(|r| r.into_iter())
            .flatten()
            .map(|s| self.walk_map("seed", "location", s).unwrap())
            .min();

        Ok(location.unwrap())
    }
}

fn main() -> Result<(), Error> {
    let almanac = Almanac::from_str(INPUT)?;

    println!("Part 1: Minimum distance to plant: {}", almanac.find_smallest_distance()?);
    println!("Part 2: Minimum distance to plant using seed ranges: {}", almanac.find_smallest_range_distance()?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

    #[test]
    fn test_part1() -> Result<(), Error> {
        let almanac = Almanac::from_str(EXAMPLE)?;
        let expected = 35;
        let actual = almanac.find_smallest_distance()?;

        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), Error> {
        let almanac = Almanac::from_str(EXAMPLE)?;
        let expected = 46;
        let actual = almanac.find_smallest_range_distance()?;

        assert_eq!(expected, actual);

        Ok(())
    }
}
