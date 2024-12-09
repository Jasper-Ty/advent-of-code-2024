advent_of_code::solution!(5);

use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp::Ordering;

// checks if list is sorted w/ mergesort strategy
// cmp is a function that returns true if x, y are in order
fn fast_check<F>(list: &Vec<u32>, cmp: F) -> bool
where
    F: Fn(u32, u32) -> bool
{
    let mut size = 1;

    while list.chunks(size*2)
            .filter(|pair| pair.len() > size)
            .all(|pair|
                (&pair[..size])
                    .iter()
                    .all(|x| cmp(*x, pair[size]))
            ) && size < list.len() {
        size *= 2;
    }

    size >= list.len()
}

type RulesMap = HashMap<u32, HashSet<u32>>;
/// Returns true if page x can come before page y, i.e there is no rule saying that y | x
fn rules_cmp(x: u32, y: u32, rules: &RulesMap) -> bool {
    rules.get(&y)
        .map(|set| set.contains(&x))
        .map_or(true, |x| !x)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut rules: RulesMap = HashMap::new();

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

    let sum = input.lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .map(|line| {
            line.split(',')
                .filter_map(|x| x.parse::<u32>().ok())
                .collect::<Vec<u32>>()
        })
        .filter(|list| fast_check(list, |x, y| rules_cmp(x, y, &rules)))
        .map(|list| list[list.len()/2])
        .fold(0u32, |acc, x| acc + x);

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut rules: HashMap<u32, HashSet<u32>> = HashMap::new();

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
            rules.entry(l)
                .or_default()
                .insert(r);
        });

    let sum = input.lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .map(|line| {
            line.split(',')
                .filter_map(|x| x.parse::<u32>().ok())
                .collect::<Vec<u32>>()
        })
        .filter(|list| !fast_check(list, |x, y| rules_cmp(x, y, &rules)))
        .map(|mut list| {
            list.sort_by(|x, y| if rules_cmp(*x, *y, &rules) { Ordering::Less } else { Ordering::Equal });
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
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
