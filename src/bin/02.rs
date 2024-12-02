advent_of_code::solution!(2);

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn are_levels_safe(levels: &Vec<i32>) -> bool {
    let deltas = levels
        .iter()
        .enumerate()
        .fold(vec![], |mut acc, (i, next)| {
            if i > 0 {
                acc.push(next - levels[i - 1]);
            }
            acc
        });

    let all_negative = deltas.iter().all(|l| *l < 0);
    let all_positive = deltas.iter().all(|l| *l > 0);
    let all_in_range = deltas.iter().all(|l| l.abs() <= 3);

    (all_negative || all_positive) && all_in_range
}

pub fn part_one(input: &str) -> Option<u32> {
    let levels = parse_input(input);

    Some(levels.iter().filter(|l| are_levels_safe(*l)).count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let levels = parse_input(input);

    Some(
        levels
            .iter()
            .filter(|l| {
                if are_levels_safe(l) {
                    true
                } else {
                    for i in 0..l.len() {
                        let mut l2 = (*l).clone();
                        l2.remove(i);
                        if are_levels_safe(&l2) {
                            return true;
                        }
                    }
                    false
                }
            })
            .count() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        assert_eq!(
            parse_input(input),
            vec![
                vec![7, 6, 4, 2, 1],
                vec![1, 2, 7, 8, 9],
                vec![9, 7, 6, 2, 1],
                vec![1, 3, 2, 4, 5],
                vec![8, 6, 4, 4, 1],
                vec![1, 3, 6, 7, 9],
            ]
        );
    }

    #[test]
    fn test_are_levels_safe() {
        assert_eq!(are_levels_safe(&vec![7, 6, 4, 2, 1]), true);
        assert_eq!(are_levels_safe(&vec![1, 2, 7, 8, 9]), false);
        assert_eq!(are_levels_safe(&vec![1, 3, 2, 4, 5]), false);
        assert_eq!(are_levels_safe(&vec![1, 3, 6, 7, 9]), true);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
