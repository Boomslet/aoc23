use std::cmp::max;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .enumerate()
        .map(|(index, line)| {
            let (_, line) = line.split_once(':')?;
            let values = line.split([',', ';']);

            for v in values {
                let (count, colour) = v.trim().split_once(' ')?;
                let count: u32 = count.parse::<u32>().unwrap();

                if colour == "blue" && count > 14
                    || colour == "red" && count > 12
                    || colour == "green" && count > 13
                {
                    return None;
                }
            }

            Some(index as u32 + 1)
        })
        .sum()
}

pub fn part_two(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|line| {
            let (_, line) = line.split_once(':')?;
            let values = line.split([',', ';']);

            let mut red = 1;
            let mut green = 1;
            let mut blue = 1;

            for v in values {
                let (count, colour) = v.trim().split_once(' ')?;
                let count: u32 = count.parse::<u32>().unwrap();

                match colour {
                    "red" => red = max(red, count),
                    "green" => green = max(green, count),
                    "blue" => blue = max(blue, count),
                    _ => {}
                }
            }

            Some(red * green * blue)
        })
        .sum()
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
