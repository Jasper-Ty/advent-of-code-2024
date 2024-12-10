advent_of_code::solution!(10);

use advent_of_code::Map;

pub fn part_one(input: &str) -> Option<u32> {
    let map = Map::new(input);

    let mut stack: Vec<(u8, (usize, usize))> = vec![];
    let mut sum = 0;
    let mut traversed = Map::empty(map.width(), map.height());

    for i in 0..map.data.len() {
        stack.clear();
        traversed.clear();
        if map.data[i] == b'9' {
            stack.push((b'9', map.pos(i)));
            while let Some((level, (x, y))) = stack.pop() {
                traversed[(x, y)] = b'X';
                if level == b'0' {
                    sum += 1;
                } else {
                    for dir in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                        if let Some((x, y)) = map.step((x, y), dir) {
                            if map[(x, y)] == level - 1 && traversed[(x, y)] != b'X' {
                                stack.push((level - 1, (x, y)));
                            }
                        }
                    }
                }
            }
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = Map::new(input);

    let mut stack: Vec<(usize, usize)> = map.data.iter()
        .enumerate()
        .filter_map(|(i, x)| (*x == b'9').then(|| map.pos(i)))
        .collect();

    let mut sum = 0;

    while let Some((x, y)) = stack.pop() {
        for dir in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            if let Some((z, w)) = map.step((x, y), dir) {
                if map[(z, w)] == map[(x, y)] - 1 {
                    if map[(z, w)] == b'0' {
                        sum += 1;
                    } else {
                        stack.push((z, w));
                    }
                }
            }
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
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
