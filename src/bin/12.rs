use std::collections::HashMap;

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<usize> {
    let reports =
        input
            .lines()
            .map(|line| line.split_once(' ').unwrap())
            .map(|(report, groups)| {
                (
                    report.chars().collect::<Vec<char>>(),
                    groups
                        .split(',')
                        .collect::<Vec<&str>>()
                        .into_iter()
                        .map(|n| n.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>(),
                )
            });

    let mut final_results = 0;
    let mut memo = HashMap::new();

    for (board, config) in reports {
        final_results += recurse(board, config, &mut memo);
    }

    Some(final_results)
}

pub fn recurse(
    board: Vec<char>,
    config: Vec<usize>,
    memo: &mut HashMap<(Vec<char>, Vec<usize>), usize>,
) -> usize {
    if memo.contains_key(&(board.clone(), config.clone())) {
        return *memo.get(&(board, config)).unwrap();
    }

    // Base case: config is empty, board is empty or no remaining #
    if config.is_empty() {
        if board.is_empty() || board.iter().all(|c| ['.', '?'].contains(c)) {
            return 1;
        }
        return 0;
    }

    if board.is_empty() && !config.is_empty() {
        return 0;
    }

    let count = match board[0] {
        '?' => {
            // choose '.' and recurse
            let count = recurse(
                board.clone().into_iter().skip(1).collect::<Vec<char>>(),
                config.clone(),
                memo,
            );

            let n = config[0];

            // too few elements or impossible config
            if board.len() < n || board.clone().into_iter().take(n).any(|c| c == '.') {
                return count;
            }

            let next_board = board.clone().into_iter().skip(n).collect::<Vec<char>>();
            let next_config = config.clone().into_iter().skip(1).collect::<Vec<usize>>();

            if !next_board.is_empty() {
                // Last group overflowed
                if next_board[0] == '#' {
                    return count;
                }

                count
                    + recurse(
                        next_board
                            .clone()
                            .into_iter()
                            .skip(1)
                            .collect::<Vec<char>>(),
                        next_config.clone(),
                        memo,
                    )
            } else {
                // handle base case
                count + recurse(next_board.clone(), next_config.clone(), memo)
            }
        }
        '#' => {
            let n = config[0];

            // too few elements or impossible config
            if board.len() < n || board.clone().iter().take(n).any(|c| c == &'.') {
                return 0;
            }

            // skip to next group
            let next_board = board.clone().into_iter().skip(n).collect::<Vec<char>>();
            let next_config = config.clone().into_iter().skip(1).collect::<Vec<usize>>();

            if !next_board.is_empty() {
                // last group overflowed
                if next_board[0] == '#' {
                    return 0;
                }

                return recurse(
                    next_board
                        .clone()
                        .into_iter()
                        .skip(1)
                        .collect::<Vec<char>>(),
                    next_config.clone(),
                    memo,
                );
            }

            // handle base cases
            recurse(next_board.clone(), next_config.clone(), memo)
        }
        '.' => recurse(
            board
                .clone()
                .into_iter()
                .skip_while(|c| c == &'.')
                .collect::<Vec<char>>(),
            config.clone(),
            memo,
        ),
        _ => unreachable!(),
    };

    memo.insert((board, config), count);
    count
}

pub fn part_two(input: &str) -> Option<usize> {
    let reports = input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(report, groups)| {
            (
                report.chars().collect::<Vec<char>>(),
                groups
                    .split(',')
                    .collect::<Vec<&str>>()
                    .into_iter()
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>(),
            )
        })
        .map(|(mut board, mut config)| {
            let pristine_board = &board.clone();
            let pristine_config = &config.clone();

            for _ in 0..4 {
                board.push('?');
                board.extend(pristine_board);
                config.extend(pristine_config);
            }

            (board, config)
        })
        .collect::<Vec<(Vec<char>, Vec<usize>)>>();

    let mut memo: HashMap<(Vec<char>, Vec<usize>), usize> = HashMap::new();
    let mut results = 0;

    for (board, config) in reports {
        results += recurse(board, config, &mut memo);
    }

    Some(results)
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
