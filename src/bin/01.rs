use std::iter;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left, mut right): (Vec<u32>, Vec<u32>) = input.lines()
        .map(|line| {
            let a = line.split_whitespace().collect::<Vec<&str>>();
            (a[0].parse::<u32>().unwrap(), a[1].parse::<u32>().unwrap())
        }).unzip();
    
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
