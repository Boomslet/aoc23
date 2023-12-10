use std::collections::{HashMap, HashSet};
advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<usize> {
    const N_CON: [char; 3] = ['|', 'F', '7'];
    const S_CON: [char; 3] = ['|', 'L', 'J'];
    const E_CON: [char; 3] = ['-', 'L', 'F'];
    const W_CON: [char; 3] = ['-', 'J', '7'];

    const N: (isize, isize, [char; 3]) = (-1, 0, N_CON);
    const S: (isize, isize, [char; 3]) = (1, 0, S_CON);
    const E: (isize, isize, [char; 3]) = (0, -1, E_CON);
    const W: (isize, isize, [char; 3]) = (0, 1, W_CON);

    let mut directions: HashMap<char, [(isize, isize, [char; 3]); 2]> = HashMap::new();

    directions.insert('|', [N, S]);
    directions.insert('-', [E, W]);
    directions.insert('L', [N, W]);
    directions.insert('J', [N, E]);
    directions.insert('7', [E, S]);
    directions.insert('F', [W, S]);

    let mut matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let n_rows = matrix.len();
    let n_cols = matrix[0].len();

    let mut c;
    let mut x;
    let mut y = 0;
    let mut x_new;
    let mut y_new;

    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let mut offsets = directions.get(&'-')?.to_vec();

    'outer: while y < n_rows {
        x = 0;

        while x < n_cols {
            c = matrix[y][x];

            if c == 'S' {
                matrix[y][x] = '-';
                seen.insert((y, x));

                while let Some((yo, xo, valid_pipes)) = offsets.pop() {
                    x_new = x.saturating_add_signed(xo);
                    y_new = y.saturating_add_signed(yo);

                    if seen.contains(&(y_new, x_new)) {
                        continue;
                    }

                    c = matrix[y_new][x_new];

                    if valid_pipes.contains(&c) {
                        y = y_new;
                        x = x_new;
                        seen.insert((y_new, x_new));
                        offsets = directions.get(&c)?.to_vec();
                    }
                }

                break 'outer;
            }

            x += 1;
        }

        y += 1;
    }

    Some(seen.len() / 2)
}

pub fn part_two(input: &str) -> Option<usize> {
    const N_CON: [char; 3] = ['|', 'F', '7'];
    const S_CON: [char; 3] = ['|', 'L', 'J'];
    const E_CON: [char; 3] = ['-', 'L', 'F'];
    const W_CON: [char; 3] = ['-', 'J', '7'];

    const N: (isize, isize, [char; 3]) = (-1, 0, N_CON);
    const S: (isize, isize, [char; 3]) = (1, 0, S_CON);
    const E: (isize, isize, [char; 3]) = (0, -1, E_CON);
    const W: (isize, isize, [char; 3]) = (0, 1, W_CON);

    let mut directions: HashMap<char, [(isize, isize, [char; 3]); 2]> = HashMap::new();

    directions.insert('|', [N, S]);
    directions.insert('-', [E, W]);
    directions.insert('L', [N, W]);
    directions.insert('J', [N, E]);
    directions.insert('7', [E, S]);
    directions.insert('F', [W, S]);

    let mut matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let n_rows = matrix.len();
    let n_cols = matrix[0].len();

    let mut c;
    let mut x;
    let mut y = 0;
    let mut x_new;
    let mut y_new;

    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let mut offsets = directions.get(&'-')?.to_vec();

    'outer: while y < n_rows {
        x = 0;

        while x < n_cols {
            c = matrix[y][x];

            if c == 'S' {
                matrix[y][x] = '-';
                seen.insert((y, x));

                while let Some((yo, xo, valid_pipes)) = offsets.pop() {
                    x_new = x.saturating_add_signed(xo);
                    y_new = y.saturating_add_signed(yo);

                    if seen.contains(&(y_new, x_new)) {
                        continue;
                    }

                    c = matrix[y_new][x_new];

                    if valid_pipes.contains(&c) {
                        y = y_new;
                        x = x_new;
                        seen.insert((y_new, x_new));
                        offsets = directions.get(&c)?.to_vec();
                    }
                }

                break 'outer;
            }

            x += 1;
        }

        y += 1;
    }

    Some(
        (0..n_rows)
            .map(|y| -> usize {
                (0..n_cols)
                    .filter(|x| !seen.contains(&(y, *x)))
                    .map(|x| {
                        (0..x)
                            .filter(|i| {
                                seen.contains(&(y, *i)) && ['|', 'J', 'L'].contains(&matrix[y][*i])
                            })
                            .count()
                            % 2
                    })
                    .sum()
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
        assert_eq!(result, Some(6823));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(415));
    }
}
