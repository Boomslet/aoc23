advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    const VALUES: [u32; 21] = [
        0, 1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768, 65536,
        131072, 262144, 524288,
    ];

    input
        .lines()
        .map(|line| {
            let (_, line) = line.split_once(':')?;
            let (winning, numbers) = line.split_once('|').unwrap();

            let winning: Vec<u32> = winning
                .trim()
                .split(' ')
                .filter(|n| !n.is_empty())
                .map(|n| n.trim().parse().unwrap())
                .collect();

            let numbers: Vec<u32> = numbers
                .trim()
                .split(' ')
                .filter(|n| !n.is_empty())
                .map(|n| n.trim().parse().unwrap())
                .collect();

            let count: u32 = numbers
                .into_iter()
                .map(|n| {
                    if winning.contains(&n) {
                        return 1;
                    }

                    0
                })
                .sum();

            VALUES.get(count as usize)
        })
        .sum()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut values: [u32; 214] = [1; 214];

    input.lines().enumerate().for_each(|(i, line)| {
        let (_, line) = line.split_once(':').unwrap();
        let (winning, numbers) = line.split_once('|').unwrap();

        let winning: Vec<u32> = winning
            .trim()
            .split(' ')
            .filter(|n| !n.is_empty())
            .map(|n| n.trim().parse().unwrap())
            .collect();

        let numbers: Vec<u32> = numbers
            .trim()
            .split(' ')
            .filter(|n| !n.is_empty())
            .map(|n| n.trim().parse().unwrap())
            .collect();

        let mut counter = 1;

        numbers.into_iter().for_each(|n| {
            if winning.contains(&n) {
                values[i + counter] += values[i];
                counter += 1;
            }
        });
    });

    Some(values.into_iter().sum())
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
