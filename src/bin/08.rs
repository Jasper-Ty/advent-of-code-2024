use std::collections::HashMap;

advent_of_code::solution!(8);

struct Map {
    data: Vec<u8>,
    width: usize,
    height: usize,
}
impl Map {
    fn new(input: &str) -> Self {
        Self {
            data: input.lines().flat_map(str::bytes).collect(),
            width: input.lines().next().unwrap().len(),
            height: input.lines().count()
        }
    }
    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut u8> {
        (x < self.width && y < self.height)
            .then(|| &mut self[(x, y)])
    }
}
use std::ops::{Index, IndexMut};
impl Index<(usize, usize)> for Map {
    type Output = u8;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.data[self.width * y + x]
    }
}
impl IndexMut<(usize, usize)> for Map {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.data[self.width * y + x]
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = Map::new(input);
    let mut antennas: HashMap<u8, Vec<(usize, usize)>> = HashMap::new();
    let mut antinodes = Map {
        data: vec![b'.'; map.width * map.height],
        width: map.width,
        height: map.height,
    };

    for x in 0..map.width {
        for y in 0..map.height {
            if matches!(map[(x, y)], b'0'..=b'9' | b'A'..=b'Z' | b'a'..=b'z') {
                antennas.entry(map[(x, y)])
                    .or_default()
                    .push((x, y));
            }
        }
    }

    let mut nantinodes = 0u32;

    for (_freq, locations) in antennas {
        for (i, (lx, ly)) in locations.iter().enumerate() {
            for (rx, ry) in &locations[i+1..] {
                let left_antinode = (2 * lx).checked_sub(*rx)
                    .zip((2 * ly).checked_sub(*ry))
                    .and_then(|(x, y)| antinodes.get_mut(x, y));

                if let Some(sq) = left_antinode {
                    nantinodes += (std::mem::replace(sq, b'#') != b'#') as u32;
                }

                let right_antinode = (2 * rx).checked_sub(*lx)
                    .zip((2 * ry).checked_sub(*ly))
                    .and_then(|(x, y)| antinodes.get_mut(x, y));

                if let Some(sq) = right_antinode {
                    nantinodes += (std::mem::replace(sq, b'#') != b'#') as u32;
                }
            }
        }
    }

    Some(nantinodes)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = Map::new(input);
    let (width, height) = (map.width, map.height);
    let mut antennas: HashMap<u8, Vec<(usize, usize)>> = HashMap::new();
    let mut antinodes = Map {
        data: vec![b'.'; map.width * map.height],
        width,
        height,
    };

    for x in 0..map.width {
        for y in 0..map.height {
            if matches!(map[(x, y)], b'0'..=b'9' | b'A'..=b'Z' | b'a'..=b'z') {
                antennas.entry(map[(x, y)])
                    .or_default()
                    .push((x, y));
            }
        }
    }

    let mut nantinodes = 0u32;

    for (_freq, locations) in antennas {
        for (i, (lx, ly)) in locations.iter().enumerate() {
            for (rx, ry) in &locations[i+1..] {
                (0..)
                    .map_while(|n| ((n+1) * lx).checked_sub(n * *rx)
                               .zip(((n+1) * ly).checked_sub(n * *ry))
                               .filter(|(x, y)| *x < width && *y < height))
                    .for_each(|(x, y)| {
                        nantinodes += (std::mem::replace(&mut antinodes[(x, y)], b'#') != b'#') as u32;
                    });
                (0..)
                    .map_while(|n| ((n+1) * rx).checked_sub(n * *lx)
                               .zip(((n+1) * ry).checked_sub(n * *ly))
                               .filter(|(x, y)| *x < width && *y < height))
                    .for_each(|(x, y)| {
                        nantinodes += (std::mem::replace(&mut antinodes[(x, y)], b'#') != b'#') as u32;
                    });
            }
        }
    }

    Some(nantinodes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
