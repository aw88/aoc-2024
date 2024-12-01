use std::iter;

advent_of_code::solution!(1);

fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    input.lines()
        .map(|line| {
            let mut cols = line.split_whitespace();
            (cols.next().unwrap().parse::<u32>().unwrap(), cols.next().unwrap().parse::<u32>().unwrap())
        })
        .unzip()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left, mut right) = parse_input(input);
    
    left.sort();
    right.sort();

    Some(iter::zip(left, right)
        .map(|(l, r)| l.abs_diff(r))
        .sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let parsed_input = parse_input(&advent_of_code::template::read_file("examples", DAY));

        assert_eq!(parsed_input, (
            vec![3, 4, 2, 1, 3, 3],
            vec![4, 3, 5, 3, 9, 3]
        ))
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
