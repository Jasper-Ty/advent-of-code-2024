use std::collections::HashMap;

advent_of_code::solution!(1);

// "01234   56789" -> (1234, 56789)
fn split_left_right(s: &str) -> Option<(u32, u32)> {
    let mut iter = s.split("   ").filter_map(|t| t.parse::<u32>().ok());

    Some((iter.next()?, iter.next()?))
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left, mut right): (Vec<u32>, Vec<u32>) =
        input.lines().filter_map(split_left_right).unzip();

    left.sort();
    right.sort();

    let sum = left
        .into_iter()
        .zip(right)
        .map(|(l, r)| l.abs_diff(r))
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let sum = input
        .lines()
        .filter_map(split_left_right)
        .fold(HashMap::new(), |mut acc, (l, r)| {
            acc.entry(l).or_insert((0, 0)).0 += 1;
            acc.entry(r).or_insert((0, 0)).1 += 1;
            acc
        })
        .into_iter()
        .map(|(v, (lfreq, rfreq))| v * lfreq * rfreq)
        .sum();

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
