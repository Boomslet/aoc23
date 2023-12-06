use std::cmp::{max, min};
advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines().peekable();

    let mut seeds = lines
        .next()
        .into_iter()
        .map(|line| line.split_once(": ").unwrap().1)
        .flat_map(|seeds| seeds.split(' '))
        .map(|seed| seed.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    while lines.peek().is_some() {
        lines.next();
        lines.next();

        let ranges = lines
            .by_ref()
            .take_while(|line| !line.is_empty())
            .map(|line| {
                line.splitn(3, ' ')
                    .map(|values| values.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>()
            })
            .collect::<Vec<Vec<u64>>>();

        seeds = seeds
            .into_iter()
            .map(|seed| {
                for r in &ranges {
                    if r[1] <= seed && seed < r[1] + r[2] {
                        return r[0] + seed.saturating_sub(r[1]);
                    };
                }

                seed
            })
            .collect();
    }

    seeds.into_iter().min()
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines().peekable();

    let mut seeds: Vec<(u64, u64)> = lines
        .next()
        .into_iter()
        .map(|line| line.split_once(": ").unwrap().1)
        .flat_map(|seeds| {
            seeds
                .split(' ')
                .map(|seed: &str| seed.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
                .chunks(2)
                .map(|chunk| (chunk[0], chunk[0] + chunk[1]))
                .collect::<Vec<(u64, u64)>>()
        })
        .collect();

    while lines.peek().is_some() {
        lines.next();
        lines.next();

        let ranges = lines
            .by_ref()
            .take_while(|line| !line.is_empty())
            .map(|line| {
                line.splitn(3, ' ')
                    .map(|values| values.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>()
            })
            .collect::<Vec<Vec<u64>>>();

        let mut new_seeds = vec![];

        'outer: while !seeds.is_empty() {
            let (start, end) = seeds.pop()?;

            for r in &ranges {
                let interval_start = max(start, r[1]);
                let interval_end = min(end, r[1] + r[2]);

                if interval_start < interval_end {
                    // store interval overlap
                    new_seeds.insert(
                        0,
                        (interval_start - r[1] + r[0], interval_end - r[1] + r[0]),
                    );

                    // place non-overlaps back into seeds
                    if interval_start > start {
                        seeds.insert(0, (start, interval_start));
                    }

                    if end > interval_end {
                        seeds.insert(0, (interval_end, end));
                    }

                    continue 'outer;
                }
            }

            new_seeds.insert(0, (start, end));
        }

        seeds = new_seeds;
    }

    seeds.into_iter().map(|(start, _)| start).min()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(457535844));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41222968));
    }
}
