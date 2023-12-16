advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<usize> {
    let matrices = input
        .split("\n\n")
        .map(|block| {
            block
                .lines()
                .map(|line| line.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>()
        })
        .collect::<Vec<Vec<Vec<char>>>>();

    Some(
        matrices
            .into_iter()
            .map(|matrix| {
                let n_rows = matrix.len();
                let n_cols = matrix[0].len();

                for left in 0..n_cols - 1 {
                    let mut l = left;
                    let mut r = left + 1;

                    while (0..n_rows).all(|row| matrix[row][l] == matrix[row][r]) {
                        if l == 0 || r == n_cols - 1 {
                            return left + 1;
                        }

                        l -= 1;
                        r += 1;
                    }
                }

                for top in 0..n_rows - 1 {
                    let mut t = top;
                    let mut b = top + 1;

                    while (0..n_cols).all(|col| matrix[t][col] == matrix[b][col]) {
                        if t == 0 || b == n_rows - 1 {
                            return (top + 1) * 100;
                        }

                        t -= 1;
                        b += 1;
                    }
                }

                0
            })
            .sum(),
    )

    // 'meta: for matrix in matrices {
    //     let mut left = 0;
    //     let mut right = 1;
    //     let mut row = 0;
    //
    //     'outer: while right < matrix[0].len() {
    //         row = 0;
    //         let mut l = left;
    //         let mut r = right;
    //
    //         while matrix[row][l] == matrix[row][r] {
    //             row += 1;
    //
    //             if row == matrix.len() {
    //                 row = 0;
    //
    //                 if l == 0 || r == matrix[row].len() - 1 {
    //                     count += right;
    //                     continue 'meta;
    //                 }
    //
    //                 l -= 1;
    //                 r += 1;
    //             }
    //         }
    //
    //         left += 1;
    //         right += 1;
    //     }
    //
    //     let mut top = 0;
    //     let mut bottom = 1;
    //
    //     'outer: while bottom < matrix.len() {
    //         let mut r1 = &matrix[top];
    //         let mut r2 = &matrix[bottom];
    //
    //         let mut t = top;
    //         let mut b = bottom;
    //
    //         while r1.iter().zip(r2.iter()).all(|(x, y)| x == y) {
    //             if t == 0 || b == matrix.len() - 1 {
    //                 count += bottom * 100;
    //                 continue 'meta;
    //             }
    //
    //             t -= 1;
    //             b += 1;
    //
    //             r1 = &matrix[t];
    //             r2 = &matrix[b];
    //         }
    //
    //         top += 1;
    //         bottom += 1;
    //     }
    // }
    //
    // Some(count)
}

pub fn part_two(input: &str) -> Option<usize> {
    let matrices = input
        .split("\n\n")
        .map(|block| {
            block
                .lines()
                .map(|line| line.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>()
        })
        .collect::<Vec<Vec<Vec<char>>>>();

    Some(
        matrices
            .into_iter()
            .map(|matrix| {
                let n_rows = matrix.len();
                let n_cols = matrix[0].len();

                'outer: for left in 0..n_cols - 1 {
                    let mut can_clean = true;
                    let mut l = left;
                    let mut r = left + 1;

                    while (0..n_rows).all(|row| {
                        if matrix[row][l] == matrix[row][r] {
                            true
                        } else if can_clean {
                            can_clean = false;
                            true
                        } else {
                            false
                        }
                    }) {
                        if l == 0 || r == n_cols - 1 {
                            if !can_clean {
                                return left + 1;
                            } else {
                                continue 'outer;
                            }
                        }

                        l -= 1;
                        r += 1;
                    }
                }

                'outer: for top in 0..n_rows - 1 {
                    let mut can_clean = true;
                    let mut t = top;
                    let mut b = top + 1;

                    while (0..n_cols).all(|col| {
                        if matrix[t][col] == matrix[b][col] {
                            true
                        } else if can_clean {
                            can_clean = false;
                            true
                        } else {
                            false
                        }
                    }) {
                        if t == 0 || b == n_rows - 1 {
                            if !can_clean {
                                return (top + 1) * 100;
                            } else {
                                continue 'outer;
                            }
                        }

                        t -= 1;
                        b += 1;
                    }
                }

                0
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
