use std::collections::HashMap;

advent_of_code::solution!(11);

const DEPTH_PART_ONE: u64 = 25;

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Stone {
    num: u64,
    depth: u64
}

fn memoized_compute(stone: Stone, map: &mut HashMap<Stone, u128>) -> u128 {
    if map.contains_key(&stone) {
        map[&stone]
    } else if stone.depth == 0 {
        1
    } else {
        let v = match stone.num {
            0 => memoized_compute(Stone { num: 1, depth: stone.depth - 1 }, map),
            num => {
                let ndigits = num.ilog10() + 1;
                if ndigits % 2 == 0 {
                    let rad = 10u64.pow(ndigits / 2);
                    let l = Stone {
                        num: num / rad,
                        depth: stone.depth - 1
                    };
                    let r = Stone {
                        num: num % rad,
                        depth: stone.depth - 1
                    };
                    memoized_compute(l, map) + memoized_compute(r, map)
                } else {
                    memoized_compute(Stone { num: num * 2024, depth: stone.depth - 1 }, map)
                }
            }
        };
        map.insert(stone, v);
        v
    }
}

pub fn part_one(input: &str) -> Option<u128> {
    let mut map: HashMap<Stone, u128> = HashMap::new();

    let sum = input.split_ascii_whitespace()
        .filter_map(|s| s.parse::<u64>().ok())
        .map(|num| Stone { num, depth: DEPTH_PART_ONE })
        .map(|stone| memoized_compute(stone, &mut map))
        .sum();

    Some(sum)
}

const DEPTH_PART_TWO: u64 = 75;

pub fn part_two(input: &str) -> Option<u128> {
    let mut map: HashMap<Stone, u128> = HashMap::new();

    let sum = input.split_ascii_whitespace()
        .filter_map(|s| s.parse::<u64>().ok())
        .map(|num| Stone { num, depth: DEPTH_PART_TWO })
        .map(|stone| memoized_compute(stone, &mut map))
        .sum();

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }
}
