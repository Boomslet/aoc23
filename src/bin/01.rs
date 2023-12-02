advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|line| {
            let mut char_iter = line.chars();
            let first = char_iter.find(|char| char.is_numeric())?;
            let last = char_iter
                .rev()
                .find(|char| char.is_numeric())
                .unwrap_or(first);

            Some(first.to_digit(10)? * 10 + last.to_digit(10)?)
        })
        .sum()
}

pub fn part_two(input: &str) -> Option<u32> {
    let numbers = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    input
        .lines()
        .map(|line| {
            let chars: Vec<char> = line.chars().collect();
            let size = chars.len();

            let mut l = 0;
            let mut r = size - 1;

            let mut first: u32 = 0;

            'outer: while l < size {
                if chars[l].is_numeric() {
                    first = chars[l].to_digit(10)?;
                    break;
                }

                for (i, num) in numbers.iter().enumerate() {
                    if l + num.len() <= size && &line.get(l..l + num.len())? == num {
                        first = 1 + i as u32;
                        break 'outer;
                    }
                }

                l += 1;
            }

            let mut last: u32 = first;

            'outer: while r > l {
                if chars[r].is_numeric() {
                    last = chars[r].to_digit(10)?;
                    break;
                }

                for (i, num) in numbers.iter().enumerate() {
                    if r + num.len() <= size && &line.get(r..r + num.len())? == num {
                        last = 1 + i as u32;
                        break 'outer;
                    }
                }

                r -= 1;
            }

            Some((first * 10) + last)
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
