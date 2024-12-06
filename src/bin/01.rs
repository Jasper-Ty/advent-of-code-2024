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

    let zipped = left.into_iter().zip(right);

    let res = zipped.fold(0u32, |sum, (left, right)| sum + left.abs_diff(right));

    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut left: HashMap<u32, usize> = HashMap::new();
    let mut right: HashMap<u32, usize> = HashMap::new();

    let insert = |x: u32, map: &mut HashMap<u32, usize>| match map.get_mut(&x) {
        Some(x) => *x += 1,
        None => {
            map.insert(x, 1);
        }
    };

    input
        .lines()
        .filter_map(split_left_right)
        .for_each(|(l, r)| {
            insert(l, &mut left);
            insert(r, &mut right);
        });

    let mut sum = 0;

    for (l, lfreq) in left.iter() {
        if let Some(rfreq) = right.get(l) {
            sum += l * (*lfreq as u32) * (*rfreq as u32);
        }
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
