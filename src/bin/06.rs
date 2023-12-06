advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<usize> {
    let mut lines = input.lines();

    let times = lines
        .next()
        .into_iter()
        .map(|line| line.split_once(':').unwrap().1)
        .flat_map(|line| line.split(' '))
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let distances = lines
        .next()
        .into_iter()
        .map(|line| line.split_once(':').unwrap().1)
        .flat_map(|line| line.split(' '))
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    Some(
        times
            .into_iter()
            .zip(distances)
            .map(|(time, distance)| {
                (1..time)
                    .reduce(|count, t| match t * (time - t) > distance {
                        true => count + 1,
                        false => count,
                    })
                    .unwrap()
            })
            .product(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();

    let time = lines
        .next()
        .into_iter()
        .map(|line| line.split_once(':').unwrap().1)
        .flat_map(|line| line.split(' '))
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
        .join("")
        .parse::<u64>()
        .unwrap();

    let distance = lines
        .next()
        .into_iter()
        .map(|line| line.split_once(':').unwrap().1)
        .flat_map(|line| line.split(' '))
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
        .join("")
        .parse::<u64>()
        .unwrap();

    (1..time).reduce(|count, t| match t * (time - t) > distance {
        true => count + 1,
        false => count,
    })
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
