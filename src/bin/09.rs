advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i32> {
    Some(
        input
            .lines()
            .map(|line| {
                line.split(' ')
                    .map(|v| v.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
            })
            .map(|mut nums| {
                let mut done = false;
                let mut right = nums.len() - 1;
                let mut sum = 0;

                while !done {
                    done = true;

                    for left in 0..right {
                        nums[left] = nums[left + 1] - nums[left];
                        done &= nums[left] == 0;
                    }

                    sum += nums[right];
                    right -= 1;
                }

                sum
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<i32> {
    Some(
        input
            .lines()
            .map(|line| {
                line.split(' ')
                    .map(|v| v.parse::<i32>().unwrap())
                    .rev()
                    .collect::<Vec<i32>>()
            })
            .map(|mut nums| {
                let mut done = false;
                let mut right = nums.len() - 1;
                let mut sum = 0;

                while !done {
                    done = true;

                    for left in 0..right {
                        nums[left] = nums[left + 1] - nums[left];
                        done &= nums[left] == 0;
                    }

                    sum += nums[right];
                    right -= 1;
                }

                sum
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2043677056));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1062));
    }
}
