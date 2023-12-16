use std::collections::{HashMap, HashSet};

advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .flat_map(|line| line.split(','))
            .map(|line| {
                line.chars().map(|c| c as u32).fold(0, |mut value, n| {
                    value += n;
                    value *= 17;
                    value %= 256;
                    value
                })
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut boxes: Vec<Vec<(&str, usize)>> = vec![];

    for _ in 0..256 {
        boxes.insert(0, vec![]);
    }

    input
        .lines()
        .flat_map(|line| line.split(','))
        .for_each(|line| {
            let split_on = match line.contains('=') {
                true => '=',
                false => '-',
            };

            let box_index = line
                .chars()
                .take_while(|c| c != &'=' && c != &'-')
                .map(|c| c as usize)
                .fold(0, |mut value, n| {
                    value += n;
                    value *= 17;
                    value %= 256;
                    value
                });

            let splits = line.split_once(split_on).unwrap();
            let lens_label = splits.0;
            let focus = splits.1.parse::<usize>().unwrap_or(0);

            if split_on == '=' {
                let index = boxes[box_index]
                    .iter()
                    .position(|(label, _)| label == &lens_label);

                if index.is_some() {
                    boxes[box_index][index.unwrap()] = (lens_label, focus);
                } else {
                    boxes[box_index].push((lens_label, focus));
                }
            } else {
                let index = boxes[box_index]
                    .iter()
                    .position(|(label, _)| label == &lens_label);

                if index.is_some() {
                    boxes[box_index].remove(index.unwrap());
                }
            }
        });

    Some(
        boxes
            .into_iter()
            .enumerate()
            .map(|(num, b)| {
                dbg!(&b);
                b.into_iter()
                    .enumerate()
                    .map(|(i, (_, focus))| (num + 1) * (i + 1) * focus)
                    .sum::<usize>()
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
