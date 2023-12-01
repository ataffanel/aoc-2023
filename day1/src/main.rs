
fn day1(input_lines: impl Iterator<Item = &'static str>) -> u64 {

    input_lines.map(|line| {
        let mut digits = line.chars().filter_map(|c| c.to_digit(10));
        let mut n = digits.next().unwrap();
        if let Some(m) = digits.last() {
            n = n*10 + m;
        } else {
            n = n*10 + n
        }
        n as u64
    }).sum()
}

static DIGITS_TXT : [&'static str; 20]= [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9"
];

fn day1_2(input_lines: impl Iterator<Item = &'static str>) -> u64 {

    input_lines.map(|mut line| {
        let mut digits = Vec::new();
        while line.len() > 0 {
            let mut found = false;
            for (i, d) in DIGITS_TXT.iter().enumerate() {
                if line.to_lowercase().starts_with(d) {
                    digits.push(i % 10);
                    line = &line[1..];
                    found = true;
                    break;
                }
            }
            if !found {
                line = &line[1..];
            }
        }

        let mut n = digits.iter().cloned().next().unwrap();
        if let Some(m) = digits.last().cloned() {
            n = n*10 + m;
        } else {
            n = n*10 + n
        }

        n as u64
    }).sum()
}

// Load input at compile time so that it is static
static INPUT: &'static str = include_str!("../input.txt");

fn main() {
    println!("Sum is {}", day1(INPUT.lines()));
    println!("Sum part 2 is {}", day1_2(INPUT.lines()));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1_example() {
        let example = r#"1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet"#;
        let expected = 142;

        let current = day1(example.lines());

        assert_eq!(expected, current);
    }

    #[test]
    fn test_part2_example() {
        let example = r#"two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen"#;
        let expected = 281;

        let current = day1_2(example.lines());

        assert_eq!(expected, current);
    }
}