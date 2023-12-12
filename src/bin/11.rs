advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u32> {
    let mut matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let max_y = matrix.len();
    let max_x = matrix[0].len();

    // Mark empty rows
    let mut galaxy_coords: Vec<(usize, usize)> = (0..max_y)
        .flat_map(|y| {
            let coords = (0..max_x)
                .filter(|x| matrix[y][*x] == '#')
                .map(|x| (y, x))
                .collect::<Vec<(usize, usize)>>();

            if coords.is_empty() {
                matrix[y] = vec!['*'; max_x];
            }

            coords
        })
        .rev()
        .collect::<Vec<(usize, usize)>>();

    // Mark empty columns
    for x in 0..max_x {
        if galaxy_coords.iter().any(|(_, cx)| cx == &x) {
            continue;
        }

        (0..max_y).for_each(|y| {
            matrix[y][x] = '*';
        });
    }

    let mut results = 0;

    while galaxy_coords.len() > 1 {
        let (gy, gx) = galaxy_coords.pop()?;

        results += galaxy_coords
            .iter()
            .map(|(y, x)| {
                let mut x_distance = 0;
                let mut y_distance = 0;
                let mut cx = gx;
                let mut cy = gy;

                while cy < *y {
                    cy += 1;

                    y_distance += match matrix[cy][cx] {
                        '*' => 2,
                        _ => 1,
                    };
                }

                while cx < *x {
                    cx += 1;

                    x_distance += match matrix[cy][cx] {
                        '*' => 2,
                        _ => 1,
                    };
                }

                while cx > *x {
                    cx -= 1;

                    x_distance += match matrix[cy][cx] {
                        '*' => 2,
                        _ => 1,
                    };
                }

                x_distance + y_distance
            })
            .sum::<u32>();
    }

    Some(results)

    // (0..max_y).into_iter().for_each(|y| {
    //     (0..max_x).into_iter().for_each(|x| {
    //         // Do galaxies on same line to the right
    //         if matrix[y][x] == '#' {}
    //     });
    // });
    //
    // while y < max_y {
    //     x = 0;
    //
    //     while x < max_x {
    //         let c = matrix[y][x];
    //
    //         if c == '#' {
    //             // Do galaxies on same line to the right
    //             if n_galaxies[y] > 1 {
    //                 let mut new_x = x + 1;
    //                 let mut x_distance = 0;
    //
    //                 while new_x < max_x {
    //                     x_distance += match matrix[y][new_x] {
    //                         '*' => 2,
    //                         _ => 1,
    //                     };
    //
    //                     if matrix[y][new_x] == '#' {
    //                         results.push(x_distance);
    //                     }
    //
    //                     new_x += 1;
    //                 }
    //             }
    //
    //             let mut new_y = y + 1;
    //             let mut y_distance = 0;
    //
    //             while new_y < max_y {
    //                 y_distance += match matrix[new_y][x] {
    //                     '*' => 2,
    //                     _ => 1,
    //                 };
    //
    //                 if n_galaxies[new_y] > 0 {
    //                     let mut new_x = x;
    //                     let mut x_distance = 0;
    //
    //                     while new_x < max_x {
    //                         if matrix[new_y][new_x] == '#' {
    //                             results.push(y_distance + x_distance);
    //                         }
    //
    //                         x_distance += match matrix[new_y][new_x] {
    //                             '*' => 2,
    //                             _ => 1,
    //                         };
    //
    //                         new_x += 1;
    //                     }
    //
    //                     if x == 0 {
    //                         new_y += 1;
    //                         continue;
    //                     }
    //
    //                     let mut new_x = x.saturating_sub(1);
    //                     let mut x_distance = 0;
    //
    //                     loop {
    //                         x_distance += match matrix[new_y][new_x] {
    //                             '*' => 2,
    //                             _ => 1,
    //                         };
    //
    //                         if matrix[new_y][new_x] == '#' {
    //                             results.push(y_distance + x_distance);
    //                         }
    //
    //                         if new_x == 0 {
    //                             break;
    //                         }
    //
    //                         new_x = new_x.saturating_sub(1);
    //                     }
    //                 }
    //
    //                 new_y += 1;
    //             }
    //         }
    //
    //         x += 1;
    //     }
    //
    //     y += 1;
    // }

    // Some(results.into_iter().sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    // let mut matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    //
    // let n_rows = matrix.len();
    // let n_cols = matrix[0].len();
    //
    // let mut x = 0;
    // let mut y = 0;
    // let mut n_galaxies = 0;
    // let mut galaxy_lines = vec![];
    //
    // while y < n_rows {
    //     x = 0;
    //     let mut seen_galaxy = false;
    //     let mut galaxies_in_row = 0;
    //
    //     while x < n_cols {
    //         let c = matrix[y][x];
    //
    //         if c == '#' {
    //             seen_galaxy = true;
    //             galaxies_in_row += 1;
    //         }
    //
    //         x += 1;
    //     }
    //
    //     galaxy_lines.push(galaxies_in_row);
    //     n_galaxies += galaxies_in_row;
    //
    //     if !seen_galaxy {
    //         x = 0;
    //
    //         while x < n_cols {
    //             matrix[y][x] = '1';
    //             x += 1;
    //         }
    //     }
    //
    //     y += 1;
    // }
    //
    // x = 0;
    // y = 0;
    //
    // while x < n_cols {
    //     y = 0;
    //     let mut seen_galaxy = false;
    //
    //     while y < n_rows {
    //         let c = matrix[y][x];
    //
    //         if c == '#' {
    //             seen_galaxy = true;
    //         }
    //
    //         y += 1;
    //     }
    //
    //     if !seen_galaxy {
    //         y = 0;
    //
    //         while y < n_rows {
    //             matrix[y][x] = '1';
    //             y += 1;
    //         }
    //     }
    //
    //     x += 1;
    // }
    //
    // x = 0;
    // y = 0;
    //
    // let mut results = vec![];
    //
    // const REPLACEMENT: u64 = 1000000;
    //
    // while y < n_rows {
    //     x = 0;
    //
    //     while x < n_cols {
    //         let c = matrix[y][x];
    //
    //         if c == '#' {
    //             // Do galaxies on same line to the right
    //             if galaxy_lines[y] > 1 {
    //                 let mut new_x = x + 1;
    //                 let mut x_distance = 0;
    //
    //                 while new_x < n_cols {
    //                     x_distance += match matrix[y][new_x] {
    //                         '1' => REPLACEMENT,
    //                         _ => 1,
    //                     };
    //
    //                     if matrix[y][new_x] == '#' {
    //                         results.push(x_distance);
    //                     }
    //
    //                     new_x += 1;
    //                 }
    //             }
    //
    //             //
    //             println!("{},{}", x, y);
    //
    //             let mut new_y = y + 1;
    //             let mut y_distance = 0;
    //
    //             while new_y < n_rows {
    //                 y_distance += match matrix[new_y][x] {
    //                     '1' => REPLACEMENT,
    //                     _ => 1,
    //                 };
    //
    //                 if galaxy_lines[new_y] > 0 {
    //                     let mut new_x = x;
    //                     let mut x_distance = 0;
    //
    //                     while new_x < n_cols {
    //                         if matrix[new_y][new_x] == '#' {
    //                             results.push(y_distance + x_distance);
    //                         }
    //
    //                         x_distance += match matrix[new_y][new_x] {
    //                             '1' => REPLACEMENT,
    //                             _ => 1,
    //                         };
    //
    //                         new_x += 1;
    //                     }
    //
    //                     if x == 0 {
    //                         new_y += 1;
    //                         continue;
    //                     }
    //
    //                     let mut new_x = x.saturating_sub(1);
    //                     let mut x_distance = 0;
    //
    //                     loop {
    //                         x_distance += match matrix[new_y][new_x] {
    //                             '1' => REPLACEMENT,
    //                             _ => 1,
    //                         };
    //
    //                         if matrix[new_y][new_x] == '#' {
    //                             results.push(y_distance + x_distance);
    //                         }
    //
    //                         if new_x == 0 {
    //                             break;
    //                         }
    //
    //                         new_x = new_x.saturating_sub(1);
    //                     }
    //                 }
    //
    //                 new_y += 1;
    //             }
    //         }
    //
    //         x += 1;
    //     }
    //
    //     y += 1;
    // }
    //
    // dbg!(&results, &results.len(), n_galaxies);
    // dbg!(results.into_iter().sum::<u64>());
    // dbg!(matrix);
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9522407));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(544723432977));
    }
}
