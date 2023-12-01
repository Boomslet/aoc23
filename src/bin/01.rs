advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;

    for line in input.lines() {
        let mut first: Option<u32> = None;
        let mut last: u32 = 0;

        for c in line.chars() {
            if c.is_numeric() {
                let digit = c.to_digit(10).unwrap();

                if first.is_none() {
                    first = Some(digit * 10);
                }

                last = digit;
            }
        }

        sum = sum + first.unwrap_or(0) + last;
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let numbers = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut sum = 0;

    for line in input.lines() {
        let chars: Vec<char> = line.chars().collect();

        let mut first: u32 = 0;
        let mut last: u32 = 0;

        let mut left = 0;
        let mut right = chars.len() - 1;

        'outer: while left < chars.len() {
            if chars[left].is_numeric() {
                first = chars[left].to_digit(10).unwrap();
                break;
            }

            for (index, num) in numbers.iter().enumerate() {
                if left + num.len() <= chars.len()
                    && &line.get(left..left + num.len()).unwrap() == num
                {
                    first = 1 + index as u32;
                    break 'outer;
                }
            }

            left += 1;
        }

        'outer: loop {
            if chars[right].is_numeric() {
                last = chars[right].to_digit(10).unwrap();
                break;
            }

            for (index, num) in numbers.iter().enumerate() {
                if right + num.len() <= chars.len()
                    && &line.get(right..right + num.len()).unwrap() == num
                {
                    last = 1 + index as u32;
                    break 'outer;
                }
            }

            if right == 0 {
                break;
            }

            right -= 1;
        }

        sum += (first * 10) + last;
    }

    Some(sum)
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
