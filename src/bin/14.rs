use std::collections::{HashMap, HashSet};

advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<usize> {
    let mut grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    tilt_north(&mut grid);

    Some(
        (0..grid.len())
            .map(|row| {
                (0..grid[0].len())
                    .map(|col| {
                        if grid[row][col] == 'O' {
                            return grid.len() - row;
                        }
                        0
                    })
                    .sum::<usize>()
            })
            .sum(),
    )
}

pub fn tilt_north(grid: &mut Vec<Vec<char>>) {
    for col in 0..grid[0].len() {
        let mut top = 0;
        let mut bottom = 1;

        while bottom < grid.len() {
            match (grid[bottom][col], grid[top][col]) {
                ('#', _) => {
                    top = bottom + 1;
                    bottom += 2;
                }
                (_, 'O') | (_, '#') => {
                    top += 1;

                    if top > bottom {
                        bottom = top + 1;
                    }
                }
                ('O', '.') => {
                    (grid[bottom][col], grid[top][col]) = (grid[top][col], grid[bottom][col]);
                    (top, bottom) = (top + 1, bottom + 1);
                }
                _ => {
                    bottom += 1;
                }
            }
        }
    }
}

pub fn tilt_west(grid: &mut Vec<Vec<char>>) {
    for row in 0..grid.len() {
        let mut left = 0;
        let mut right = 1;

        while right < grid.len() {
            match (grid[row][left], grid[row][right]) {
                (_, '#') => {
                    left = right + 1;
                    right += 2;
                }
                ('O', _) | ('#', _) => {
                    left += 1;

                    if left > right {
                        right = left + 1;
                    }
                }
                ('.', 'O') => {
                    (grid[row][left], grid[row][right]) = (grid[row][right], grid[row][left]);
                    (left, right) = (left + 1, right + 1);
                }
                _ => {
                    right += 1;
                }
            }
        }
    }
}

pub fn tilt_east(grid: &mut Vec<Vec<char>>) {
    grid.iter_mut().for_each(|row| row.reverse());
    tilt_west(grid);
    grid.iter_mut().for_each(|row| row.reverse());
}

pub fn tilt_south(grid: &mut Vec<Vec<char>>) {
    grid.reverse();
    tilt_north(grid);
    grid.reverse();
}

pub fn get_state(grid: &Vec<Vec<char>>) -> String {
    grid.iter().map(String::from_iter).collect::<String>()
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut seen = HashMap::new();

    let mut grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut n = 1;
    let mut m = 1000000000;

    while n <= m {
        tilt_north(&mut grid);
        tilt_west(&mut grid);
        tilt_south(&mut grid);
        tilt_east(&mut grid);

        if seen.contains_key(&get_state(&grid)) {
            m = (m - n) % (n - seen.get(&get_state(&grid)).unwrap());
            n = 1;
            continue;
        }

        seen.insert(grid.iter().map(String::from_iter).collect::<String>(), n);

        n += 1;
    }

    Some(
        (0..grid.len())
            .map(|row| {
                (0..grid[0].len())
                    .map(|col| {
                        if grid[row][col] == 'O' {
                            return grid.len() - row;
                        }
                        0
                    })
                    .sum::<usize>()
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(110090));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
