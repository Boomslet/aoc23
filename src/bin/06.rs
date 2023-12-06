advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines().peekable();

    let times = lines
        .next()
        .into_iter()
        .map(|line| line.split_once(':').unwrap().1)
        .flat_map(|line| line.split(' '))
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let distances = lines
        .next()
        .into_iter()
        .map(|line| line.split_once(':').unwrap().1)
        .flat_map(|line| line.split(' '))
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let counts = times
        .into_iter()
        .zip(distances)
        .map(|(t, d)| {
            let count: u32 = (1..t)
                .map(|time| {
                    if time * (t - time) > d {
                        return 1;
                    }
                    0
                })
                .sum();

            count
        })
        .collect::<Vec<u32>>();

    Some(counts.into_iter().product())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines().peekable();

    let times = lines
        .next()
        .into_iter()
        .map(|line| line.split_once(':').unwrap().1)
        .flat_map(|line| line.split(' '))
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
        .join("");

    let time = times.parse::<u64>().unwrap();

    let distances = lines
        .next()
        .into_iter()
        .map(|line| line.split_once(':').unwrap().1)
        .flat_map(|line| line.split(' '))
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
        .join("");

    let distance = distances.parse::<u64>().unwrap();

    let count: u64 = (1..time)
        .map(|t| {
            if t * (time - t) > distance {
                return 1;
            }
            0
        })
        .sum();

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6209190));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(28545089));
    }
}
