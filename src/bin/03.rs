use once_cell::sync::Lazy;
use regex::Regex;

advent_of_code::solution!(3);

fn parse_mul(input: &str) -> Option<u32> {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"mul\((\d+),(\d+)\)").unwrap());
    if let Some(captures) = RE.captures(input) {
        Some(&captures[1].parse::<u32>().unwrap() * &captures[2].parse::<u32>().unwrap())
    } else {
        None
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut sum = 0;
    for (_, [a, b]) in re.captures_iter(input).map(|c| c.extract()) {
        sum = sum + a.parse::<u32>().unwrap() * b.parse::<u32>().unwrap();
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"(don't\(\)|do\(\)|mul\(\d+,\d+\))").unwrap();

    let mut sum = 0;
    let mut enabled = true;
    for (_, [command]) in re.captures_iter(input).map(|c| c.extract()) {
        match command {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ => {
                if enabled {
                    sum = sum + parse_mul(command).unwrap();
                }
            }
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
