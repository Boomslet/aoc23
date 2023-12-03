advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    const SYMBOLS: [char; 10] = ['#', '$', '%', '&', '*', '+', '-', '/', '=', '@'];

    let mut matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let n_rows = matrix.len();
    let n_cols = matrix[0].len();

    let mut x;
    let mut y = 0;

    let mut total = 0;

    while y < n_rows {
        x = 0;

        while x < n_cols {
            let c = matrix[y][x];

            if SYMBOLS.contains(&c) {
                for y_offset in [y.saturating_sub(1), y, y + 1] {
                    if y_offset == n_rows {
                        continue;
                    }

                    for mut x_offset in [x.saturating_sub(1), x, x + 1] {
                        if x_offset == n_cols {
                            continue;
                        }

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
                                matrix[y_offset][x_offset] = '.';
                                // seen.insert((y_offset, x_offset));
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
    let mut matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let n_rows = matrix.len();
    let n_cols = matrix[0].len();

    let mut x;
    let mut y = 0;

    let mut total = 0;

    while y < n_rows {
        x = 0;

        while x < n_cols {
            let c = matrix[y][x];

            if c == '*' {
                let mut sums: Vec<u32> = vec![];

                for y_offset in [y.saturating_sub(1), y, y + 1] {
                    if y_offset == n_rows {
                        continue;
                    }

                    for mut x_offset in [x.saturating_sub(1), x, x + 1] {
                        if x_offset == n_cols {
                            continue;
                        }

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
                                matrix[y_offset][x_offset] = '.';
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
        assert_eq!(result, Some(540131));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(86879020));
    }
}
