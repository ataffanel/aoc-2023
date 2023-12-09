use anyhow::Error;
use itertools::Itertools;
use std::str::FromStr;

struct Sequence {
    history: Vec<i64>,
}

impl FromStr for Sequence {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let history = s.split(" ").map(i64::from_str).collect::<Result<_, _>>()?;

        Ok(Self { history })
    }
}

impl Sequence {
    fn extrapolate(&self) -> i64 {
        let mut history = self.history.clone();
        let mut last_derivates = Vec::new();

        loop {
            history = history
                .iter()
                .tuple_windows::<(_, _)>()
                .map(|(prev, next)| next - prev)
                .collect();

            let last = history.iter().cloned().last().unwrap();
            last_derivates.push(last);

            if history.iter().all(|d| *d == 0) {
                break;
            }
        }
        last_derivates.iter().sum::<i64>() + *self.history.last().unwrap()
    }

    fn back_extrapolate(&self) -> i64 {
        let mut history = self.history.clone();
        let mut first_derivates = Vec::new();

        loop {
            history = history
                .iter()
                .tuple_windows::<(_, _)>()
                .map(|(prev, next)| next - prev)
                .collect();

            let first = history.iter().cloned().next().unwrap();
            first_derivates.push(first);

            // dbg!(&history);

            if history.iter().all(|d| *d == 0) {
                break;
            }
        }
        *self.history.first().unwrap() - first_derivates.iter().rev().fold(0, |p, d| d - p)
    }
}

struct Report {
    sequences: Vec<Sequence>,
}

impl FromStr for Report {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sequences = s
            .lines()
            .map(Sequence::from_str)
            .collect::<Result<_, _>>()?;

        Ok(Report { sequences })
    }
}

impl Report {
    fn extrapolation_sum(&self) -> i64 {
        self.sequences.iter().map(|s| s.extrapolate()).sum()
    }

    fn back_extrapolation_sum(&self) -> i64 {
        self.sequences.iter().map(|s| s.back_extrapolate()).sum()
    }
}

static INPUT: &str = include_str!("../input.txt");

fn main() -> Result<(), Error> {
    let report = Report::from_str(INPUT)?;

    println!(
        "Part 1: Sum of the predictions: {}",
        report.extrapolation_sum()
    );
    println!(
        "Part 2: Sum of the back-predictions: {}",
        report.back_extrapolation_sum()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;

    #[test]
    fn test_example_parsing() -> Result<(), Error> {
        let report = Report::from_str(EXAMPLE)?;
        let expected_sequences = 3;
        let expected_seq_len = 6;

        assert_eq!(expected_sequences, report.sequences.len());
        assert_eq!(expected_seq_len, report.sequences[0].history.len());

        Ok(())
    }

    #[test]
    fn test_extrapolate_forward_result() -> Result<(), Error> {
        let report = Report::from_str(EXAMPLE)?;
        let expected = 114;

        let actual = report.extrapolation_sum();

        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn test_extrapolate_backward_result() -> Result<(), Error> {
        let report = Report::from_str(EXAMPLE)?;
        let expected = 2;

        let actual = report.back_extrapolation_sum();

        assert_eq!(expected, actual);

        Ok(())
    }
}
