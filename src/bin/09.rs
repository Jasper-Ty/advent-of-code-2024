advent_of_code::solution!(9);

use std::iter::repeat;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Block {
    File(usize),
    Empty
}
use Block::*;

pub fn part_one(input: &str) -> Option<usize> {
    let mut disk: Vec<Block> = vec![];

    for (i, n) in input.chars().filter_map(|ch| ch.to_digit(10)).enumerate() {
        if i % 2 == 0 {
            disk.extend(repeat(File(i/2)).take(n as usize));
        } else {
            disk.extend(repeat(Empty).take(n as usize));
        }
    }

    let mut l: usize = 0;
    let mut r: usize = disk.len() - 1;

    loop {
        while disk[l] != Empty {
            l += 1;
        }
        while disk[r] == Empty {
            r -= 1;
        }
        if l >= r {
            break;
        }
        disk.swap(l, r);
    }

    let checksum = disk
        .iter()
        .enumerate()
        .filter_map(|(i, block)| match block {
            File(n) => Some(i * n),
            Empty => None
        })
        .sum();

    Some(checksum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut disk: Vec<Block> = vec![];

    for (i, n) in input.chars().filter_map(|ch| ch.to_digit(10)).enumerate() {
        if i % 2 == 0 {
            disk.extend(repeat(File(i/2)).take(n as usize));
        } else {
            disk.extend(repeat(Empty).take(n as usize));
        }
    }

    let mut r = disk.len() - 1;
    loop {
        while disk[r] == Empty {
            r -= 1;
        }
        if disk[r] == File(0) {
            break;
        }
        let mut rlen = 0;
        while disk[r-rlen] == disk[r] {
            rlen += 1;
        }

        // find empty block
        let mut l = 0;
        let canmove = loop {
            // find next empty block
            while disk[l] != Empty {
                l += 1;
            }
            if l >= r {
                break false;
            }
            // see how long it is
            let mut llen = 0;
            while let Some(Empty) = disk.get(l+llen) {
                llen += 1;
            }

            // break if its good
            if llen >= rlen {
                break true;
            }

            // else keep going
            l += llen;
        };

        if canmove {
            for _ in 0..rlen {
                disk.swap(l, r);
                l += 1;
                r -= 1;
            }
        } else {
            r -= rlen;
        }

    }

    let checksum = disk
        .iter()
        .enumerate()
        .filter_map(|(i, block)| match block {
            File(n) => Some(i * n),
            Empty => None
        })
        .sum();

    Some(checksum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
