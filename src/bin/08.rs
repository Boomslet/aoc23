use gcd::Gcd;
use std::collections::HashMap;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<usize> {
    let mut lines = input.lines();

    let mut directions: [usize; 307] = [0; 307];
    let mut d_index = 0;

    lines
        .next()
        .unwrap()
        .chars()
        .enumerate()
        .for_each(|(i, c)| {
            directions[i] = match c {
                'L' => 0,
                'R' => 1,
                _ => unreachable!(),
            };
        });

    // Remove blank line
    lines.next();

    let mut node_map = HashMap::new();

    lines
        .map(|line| line.split_once(" = ").unwrap())
        .map(|(node, connections)| (node, connections.splitn(2, ", ").collect::<Vec<&str>>()))
        .map(|(n, c)| {
            (
                n,
                vec![
                    c[0].chars().skip(1).collect::<String>(),
                    c[1].chars().take(3).collect::<String>(),
                ],
            )
        })
        .for_each(|(n, c)| {
            node_map.insert(n, c);
        });

    let mut result = 0;
    let mut target = "AAA";

    while target != "ZZZ" {
        target = &node_map.get(target)?[directions[d_index]];
        result += 1;
        d_index = (d_index + 1) % 307;
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut lines = input.lines();
    let mut directions: [usize; 307] = [0; 307];
    let mut d_index;

    lines
        .next()
        .unwrap()
        .chars()
        .enumerate()
        .for_each(|(i, c)| {
            directions[i] = match c {
                'L' => 0,
                'R' => 1,
                _ => unreachable!(),
            };
        });

    // Remove blank line
    lines.next();

    let mut node_map = HashMap::new();
    let mut spooky_nodes = vec![];

    lines
        .map(|line| line.split_once(" = ").unwrap())
        .map(|(node, connections)| (node, connections.splitn(2, ", ").collect::<Vec<&str>>()))
        .map(|(n, c)| {
            (
                n,
                vec![
                    c[0].chars().skip(1).collect::<String>(),
                    c[1].chars().take(3).collect::<String>(),
                ],
            )
        })
        .for_each(|(n, c)| {
            node_map.insert(n, c);

            if n.ends_with('A') {
                spooky_nodes.push(n);
            }
        });

    let mut z_reps = vec![0; spooky_nodes.len()];
    let mut count: usize;

    for (i, mut node) in spooky_nodes.into_iter().enumerate() {
        count = 0;
        d_index = 0;

        while !node.ends_with('Z') {
            count += 1;
            node = &node_map.get(node).unwrap()[directions[d_index]];
            d_index = (d_index + 1) % 307;
        }

        z_reps[i] = count;
    }

    z_reps
        .into_iter()
        .reduce(|acc, node| (acc * node) / node.gcd(acc))
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
