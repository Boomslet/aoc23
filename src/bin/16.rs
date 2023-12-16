use std::collections::HashSet;

advent_of_code::solution!(16);

pub fn part_one(input: &str) -> Option<usize> {
    let matrix = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let n_rows = matrix.len();
    let n_cols = matrix[0].len();

    const UP: (i32, i32) = (0, -1);
    const DOWN: (i32, i32) = (0, 1);
    const LEFT: (i32, i32) = (-1, 0);
    const RIGHT: (i32, i32) = (1, 0);

    let mut directions = vec![((0, 0), RIGHT)];
    let mut seen: HashSet<((i32, i32), (i32, i32))> = HashSet::new();

    while let Some((coords, direction)) = directions.pop() {
        if coords.0 < 0 || coords.0 as usize >= n_cols {
            continue;
        }

        if coords.1 < 0 || coords.1 as usize >= n_rows {
            continue;
        }

        if seen.contains(&(coords, direction)) {
            continue;
        }

        seen.insert((coords, direction));

        match matrix[coords.1 as usize][coords.0 as usize] {
            '\\' => match direction {
                UP => directions.push((steer(coords, LEFT), LEFT)),
                DOWN => directions.push((steer(coords, RIGHT), RIGHT)),
                LEFT => directions.push((steer(coords, UP), UP)),
                RIGHT => directions.push((steer(coords, DOWN), DOWN)),
                _ => unreachable!(),
            },
            '/' => match direction {
                UP => directions.push((steer(coords, RIGHT), RIGHT)),
                DOWN => directions.push((steer(coords, LEFT), LEFT)),
                LEFT => directions.push((steer(coords, DOWN), DOWN)),
                RIGHT => directions.push((steer(coords, UP), UP)),
                _ => unreachable!(),
            },
            '|' => match direction {
                LEFT | RIGHT => {
                    directions.push((steer(coords, UP), UP));
                    directions.push((steer(coords, DOWN), DOWN));
                }
                UP | DOWN => {
                    directions.push((steer(coords, direction), direction));
                }
                _ => unreachable!(),
            },
            '-' => match direction {
                UP | DOWN => {
                    directions.push((steer(coords, LEFT), LEFT));
                    directions.push((steer(coords, RIGHT), RIGHT));
                }
                LEFT | RIGHT => {
                    directions.push((steer(coords, direction), direction));
                }
                _ => unreachable!(),
            },
            _ => {
                directions.push((steer(coords, direction), direction));
            }
        }
    }

    let mut result = seen
        .into_iter()
        .map(|(c, _)| c)
        .collect::<Vec<(i32, i32)>>();

    result.sort();
    result.dedup();
    Some(result.len())
}

pub fn steer(input: (i32, i32), direction: (i32, i32)) -> (i32, i32) {
    (input.0 + direction.0, input.1 + direction.1)
}

pub fn part_two(input: &str) -> Option<usize> {
    let matrix = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let n_rows = matrix.len();
    let n_cols = matrix[0].len();

    const UP: (i32, i32) = (0, -1);
    const DOWN: (i32, i32) = (0, 1);
    const LEFT: (i32, i32) = (-1, 0);
    const RIGHT: (i32, i32) = (1, 0);

    let mut best = 0;

    for column in 0..n_cols {
        let current = illuminate(&matrix, vec![((column as i32, 0), DOWN)]);

        if current > best {
            best = current;
        }

        let current = illuminate(&matrix, vec![((column as i32, (n_rows - 1) as i32), UP)]);

        if current > best {
            best = current;
        }
    }

    for row in 0..n_rows {
        let current = illuminate(&matrix, vec![((0, row as i32), RIGHT)]);

        if current > best {
            best = current;
        }

        let current = illuminate(&matrix, vec![(((n_cols - 1) as i32, row as i32), LEFT)]);

        if current > best {
            best = current;
        }
    }

    Some(best)
}

pub fn illuminate(matrix: &Vec<Vec<char>>, mut directions: Vec<((i32, i32), (i32, i32))>) -> usize {
    let n_rows = matrix.len();
    let n_cols = matrix[0].len();

    const UP: (i32, i32) = (0, -1);
    const DOWN: (i32, i32) = (0, 1);
    const LEFT: (i32, i32) = (-1, 0);
    const RIGHT: (i32, i32) = (1, 0);

    let mut seen: HashSet<((i32, i32), (i32, i32))> = HashSet::new();

    while let Some((coords, direction)) = directions.pop() {
        if coords.0 < 0 || coords.0 as usize >= n_cols {
            continue;
        }

        if coords.1 < 0 || coords.1 as usize >= n_rows {
            continue;
        }

        if seen.contains(&(coords, direction)) {
            continue;
        }

        seen.insert((coords, direction));

        match matrix[coords.1 as usize][coords.0 as usize] {
            '\\' => match direction {
                UP => directions.push((steer(coords, LEFT), LEFT)),
                DOWN => directions.push((steer(coords, RIGHT), RIGHT)),
                LEFT => directions.push((steer(coords, UP), UP)),
                RIGHT => directions.push((steer(coords, DOWN), DOWN)),
                _ => unreachable!(),
            },
            '/' => match direction {
                UP => directions.push((steer(coords, RIGHT), RIGHT)),
                DOWN => directions.push((steer(coords, LEFT), LEFT)),
                LEFT => directions.push((steer(coords, DOWN), DOWN)),
                RIGHT => directions.push((steer(coords, UP), UP)),
                _ => unreachable!(),
            },
            '|' => match direction {
                LEFT | RIGHT => {
                    directions.push((steer(coords, UP), UP));
                    directions.push((steer(coords, DOWN), DOWN));
                }
                UP | DOWN => {
                    directions.push((steer(coords, direction), direction));
                }
                _ => unreachable!(),
            },
            '-' => match direction {
                UP | DOWN => {
                    directions.push((steer(coords, LEFT), LEFT));
                    directions.push((steer(coords, RIGHT), RIGHT));
                }
                LEFT | RIGHT => {
                    directions.push((steer(coords, direction), direction));
                }
                _ => unreachable!(),
            },
            _ => {
                directions.push((steer(coords, direction), direction));
            }
        }
    }

    let mut result = seen
        .into_iter()
        .map(|(c, _)| c)
        .collect::<Vec<(i32, i32)>>();

    result.sort();
    result.dedup();
    result.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
