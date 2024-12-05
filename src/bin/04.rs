advent_of_code::solution!(4);

#[derive(Debug)]
struct IVec2(i32, i32);

impl IVec2 {
    fn add(&self, rhs: &IVec2) -> IVec2 {
        IVec2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn find_xmases(
    start: &IVec2,
    grid: &Vec<Vec<char>>,
    word: &str,
    direction: Option<&IVec2>,
) -> usize {
    let directions = vec![
        IVec2(1, 0),   // East
        IVec2(1, 1),   // Southeast
        IVec2(0, 1),   // South
        IVec2(-1, 1),  // Southwest
        IVec2(-1, 0),  // West
        IVec2(-1, -1), // Northwest
        IVec2(0, -1),  // North
        IVec2(1, -1),  // Northeast
    ];

    if start.0 < 0 || start.1 < 0 || start.0 >= grid.len() as i32 || start.1 >= grid[0].len() as i32
    {
        0
    } else if grid[start.0 as usize][start.1 as usize] == word.chars().next().unwrap() {
        if word.len() == 1 {
            1
        } else {
            let next_word = word.split_at(1).1;

            if let Some(next_direction) = direction {
                let next_start = start.add(next_direction);
                find_xmases(&next_start, grid, next_word, direction)
            } else {
                directions
                    .iter()
                    .map(|d| {
                        let next_start = start.add(d);
                        find_xmases(&next_start, grid, next_word, Some(&d))
                    })
                    .sum()
            }
        }
    } else {
        0
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_input(input);

    let mut count = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            count = count + find_xmases(&IVec2(x as i32, y as i32), &grid, "XMAS", None);
        }
    }

    Some(count as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let grid = parse_input(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(
            grid,
            vec![
                vec!['M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M'],
                vec!['M', 'S', 'A', 'M', 'X', 'M', 'S', 'M', 'S', 'A'],
                vec!['A', 'M', 'X', 'S', 'X', 'M', 'A', 'A', 'M', 'M'],
                vec!['M', 'S', 'A', 'M', 'A', 'S', 'M', 'S', 'M', 'X'],
                vec!['X', 'M', 'A', 'S', 'A', 'M', 'X', 'A', 'M', 'M'],
                vec!['X', 'X', 'A', 'M', 'M', 'X', 'X', 'A', 'M', 'A'],
                vec!['S', 'M', 'S', 'M', 'S', 'A', 'S', 'X', 'S', 'S'],
                vec!['S', 'A', 'X', 'A', 'M', 'A', 'S', 'A', 'A', 'A'],
                vec!['M', 'A', 'M', 'M', 'M', 'X', 'M', 'M', 'M', 'M'],
                vec!['M', 'X', 'M', 'X', 'A', 'X', 'M', 'A', 'S', 'X'],
            ]
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
