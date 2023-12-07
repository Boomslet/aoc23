advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<usize> {
    let mut results = input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(hand, bid)| {
            (
                hand.chars()
                    .map(|c| match c {
                        'A' => 12,
                        'K' => 11,
                        'Q' => 10,
                        'J' => 9,
                        'T' => 8,
                        '9' => 7,
                        '8' => 6,
                        '7' => 5,
                        '6' => 4,
                        '5' => 3,
                        '4' => 2,
                        '3' => 1,
                        '2' => 0,
                        _ => unreachable!(),
                    })
                    .collect::<Vec<usize>>(),
                bid.parse::<usize>().unwrap(),
            )
        })
        .map(|(hand, bid)| -> (usize, Vec<usize>, usize) {
            let mut frequencies: [usize; 13] = [0; 13];

            hand.iter().for_each(|card| frequencies[*card] += 1);

            let frequencies = frequencies
                .into_iter()
                .filter(|f| f > &1)
                .collect::<Vec<usize>>();

            let strength = match (frequencies.iter().sum(), frequencies.len()) {
                (5, 1) => 7,
                (4, 1) => 6,
                (5, 2) => 5,
                (3, 1) => 4,
                (4, 2) => 3,
                (2, 1) => 2,
                _ => 1,
            };

            (strength, hand, bid)
        })
        .collect::<Vec<(usize, Vec<usize>, usize)>>();

    results.sort_unstable();

    Some(
        results
            .into_iter()
            .enumerate()
            .rev()
            .map(|(index, (_, _, bid))| bid * (index + 1))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut results = input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(hand, bid)| {
            (
                hand.chars()
                    .map(|c| match c {
                        'A' => 12,
                        'K' => 11,
                        'Q' => 10,
                        'J' => 0,
                        'T' => 9,
                        '9' => 8,
                        '8' => 7,
                        '7' => 6,
                        '6' => 5,
                        '5' => 4,
                        '4' => 3,
                        '3' => 2,
                        '2' => 1,
                        _ => unreachable!(),
                    })
                    .collect::<Vec<usize>>(),
                bid.parse::<usize>().unwrap(),
            )
        })
        .map(|(hand, bid)| -> (usize, Vec<usize>, usize) {
            let mut frequencies: [usize; 13] = [0; 13];

            hand.iter().for_each(|card| frequencies[*card] += 1);

            let jacks = frequencies[0];

            frequencies[0] = 0;

            let frequencies = frequencies
                .into_iter()
                .filter(|f| f > &1)
                .collect::<Vec<usize>>();

            let strength = match (frequencies.iter().sum::<usize>() + jacks, frequencies.len()) {
                (5, 1) => 7,
                (5, 0) => 7,
                (4, 0) => 7,
                (4, 1) => 6,
                (3, 0) => 6,
                (5, 2) => 5,
                (3, 1) => 4,
                (2, 0) => 4,
                (4, 2) => 3,
                (2, 1) => 2,
                (1, 0) => 2,
                _ => 1,
            };

            (strength, hand, bid)
        })
        .collect::<Vec<(usize, Vec<usize>, usize)>>();

    results.sort_unstable();

    Some(
        results
            .into_iter()
            .enumerate()
            .rev()
            .map(|(index, (_, _, bid))| bid * (index + 1))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(253205868));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(253907829));
    }
}
