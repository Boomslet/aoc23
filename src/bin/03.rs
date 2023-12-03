use std::collections::HashSet;
advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let symbols = ['#', '$', '%', '&', '*', '+', '-', '/', '=', '@'];

    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let n_rows = matrix.len();
    let n_cols = matrix[0].len();

    let mut x;
    let mut y = 0;

    let mut seen = HashSet::new();
    let mut total = 0;

    while y < n_rows {
        x = 0;

        while x < n_cols {
            if seen.contains(&(y, x)) {
                x += 1;
                continue;
            }

            let c = matrix[y][x];

            if symbols.contains(&c) {
                let mut y_offsets = vec![y, y + 1];
                let mut x_offsets = vec![x, x + 1];

                if y > 0 {
                    y_offsets.insert(0, y - 1);
                }

                if x > 0 {
                    x_offsets.insert(0, x - 1);
                }

                for y_offset in y_offsets {
                    for mut x_offset in x_offsets.clone() {
                        if x_offset >= n_cols
                            || y_offset >= n_rows
                            || seen.contains(&(y_offset, x_offset))
                        {
                            continue;
                        }

                        seen.insert((y_offset, x_offset));

                        if matrix[y_offset][x_offset].is_numeric() {
                            while matrix[y_offset][x_offset - 1].is_numeric() {
                                x_offset -= 1;

                                if x_offset == 0 {
                                    break;
                                }
                            }

                            let mut sum = 0;

                            while x_offset < n_cols && matrix[y_offset][x_offset].is_numeric() {
                                sum *= 10;
                                sum += matrix[y_offset][x_offset].to_digit(10)?;
                                seen.insert((y_offset, x_offset));
                                x_offset += 1;
                            }

                            total += sum;
                        }
                    }
                }
            }

            x += 1;
        }

        y += 1;
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let n_rows = matrix.len();
    let n_cols = matrix[0].len();

    let mut x;
    let mut y = 0;

    let mut seen = HashSet::new();
    let mut total = 0;

    while y < n_rows {
        x = 0;

        while x < n_cols {
            if seen.contains(&(y, x)) {
                x += 1;
                continue;
            }

            let c = matrix[y][x];

            if c == '*' {
                let mut y_offsets = vec![y, y + 1];
                let mut x_offsets = vec![x, x + 1];

                if y > 0 {
                    y_offsets.insert(0, y - 1);
                }

                if x > 0 {
                    x_offsets.insert(0, x - 1);
                }

                let mut sums: Vec<u32> = vec![];

                for y_offset in y_offsets {
                    for mut x_offset in x_offsets.clone() {
                        if x_offset >= n_cols
                            || y_offset >= n_rows
                            || seen.contains(&(y_offset, x_offset))
                        {
                            continue;
                        }

                        seen.insert((y_offset, x_offset));

                        if matrix[y_offset][x_offset].is_numeric() {
                            while matrix[y_offset][x_offset - 1].is_numeric() {
                                x_offset -= 1;

                                if x_offset == 0 {
                                    break;
                                }
                            }

                            let mut sum = 0;

                            while x_offset < n_cols && matrix[y_offset][x_offset].is_numeric() {
                                sum *= 10;
                                sum += matrix[y_offset][x_offset].to_digit(10)?;
                                seen.insert((y_offset, x_offset));
                                x_offset += 1;
                            }

                            sums.insert(0, sum);
                        }
                    }
                }

                if sums.len() == 2 {
                    total += sums[0] * sums[1];
                }
            }

            x += 1;
        }

        y += 1;
    }

    Some(total)
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
