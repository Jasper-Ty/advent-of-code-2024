advent_of_code::solution!(5);

use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp::Ordering;

// modified mergesort
fn fast_check<F>(list: Vec<u32>, cmp: F) -> bool
where
    F: Fn(u32, u32) -> bool
{
    let n = list.len();

    let mut size = 1;
    while size < n {
        for pair in list.chunks(size*2) {
            if pair.len() < size {
                continue;
            }
            let left = &pair[..size];
            let right = &pair[size..];
        }
        size *= 2;
    }

    false
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut rules: HashMap<u32, HashSet<u32>> = HashMap::new();

    let bytes: Vec<u8> = input
        .lines()
        .flat_map(str::bytes)
        .collect();

    let mut slice = input.as_bytes();
    loop {
        match slice {
            [a @ b'0'..=b'9', b @ b'0'..=b'9', b'|', c @ b'0'..=b'9', d @ b'0'..=b'9', b'\n', rest @ ..] => {
                let lhs = (*a as u32 - b'0' as u32) * 10 + (*b as u32 - b'0' as u32);
                let rhs = (*c as u32 - b'0' as u32) * 10 + (*d as u32 - b'0' as u32);
                rules.entry(lhs)
                    .or_default()
                    .insert(rhs);
                slice = rest;
            }
            _ => break
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut rules_lt: HashMap<u32, HashSet<u32>> = HashMap::new();
    let mut rules_gt: HashMap<u32, HashSet<u32>> = HashMap::new();

    input.lines()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let mut iter = line.split('|');
            (
                iter.next().unwrap().parse::<u32>().unwrap(),
                iter.next().unwrap().parse::<u32>().unwrap()
            )
        })
        .for_each(|(l, r)| {
            rules_lt.entry(l)
                .or_default()
                .insert(r);
            rules_gt.entry(r)
                .or_default()
                .insert(l);
        });

    let sum = input.lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .map(|line| {
            line.split(',')
                .filter_map(|x| x.parse::<u32>().ok())
                .collect::<Vec<u32>>()
        })
        .filter(|list| {
            let map: HashMap<u32, usize> = list
                .iter()
                .enumerate()
                .map(|(x, y)| (*y, x))
                .collect();
            !list
                .iter()
                .enumerate()
                .all(|(idx, x)|
                    rules_lt[x]
                        .iter()
                        .filter_map(|y| map.get(y))
                        .all(|idy| idx < *idy)
                    &&
                    rules_gt[x]
                        .iter()
                        .filter_map(|y| map.get(y))
                        .all(|idy| idx > *idy)
                )
        })
        .map(|mut list| {
            list.sort_by(|x, y| if rules_lt[x].contains(y) { Ordering::Less } else { Ordering::Equal });
            list[list.len()/2]
        })
        .fold(0u32, |acc, x| acc + x);

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
