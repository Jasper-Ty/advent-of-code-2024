advent_of_code::solution!(6);

use std::str;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right
}
use Dir::*;
impl Dir {
    fn step(&self, x: usize, y: usize, m: usize, n: usize) -> Option<(usize, usize)> {
        match self {
            Up => (y > 0).then(|| (x,y-1)),
            Down => (y < m-1).then(|| (x,y+1)),
            Left => (x > 0).then(|| (x-1,y)),
            Right => (x < n-1).then(|| (x+1,y)),
        }
    }
    fn ccw(&self) -> Self {
        match self {
            Up => Left,
            Down => Right,
            Left => Down,
            Right => Up
        }
    }
    fn turn_ccw(&mut self) {
        *self = self.ccw();
    }
    fn cw(&self) -> Self {
        match self {
            Up => Right,
            Down => Left,
            Left => Up,
            Right => Down
        }
    }
    fn turn_cw(&mut self) {
        *self = self.cw();
    }
    fn iter() -> impl Iterator<Item=Self> {
        [Up, Down, Left, Right].iter().cloned()
    }
}

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
    fn print(&self) {
       self.data[..]
            .chunks(self.width)
            .filter_map(|slice| str::from_utf8(slice).ok())
            .for_each(|s| println!("{}", s));
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

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Pos {
    x: usize,
    y: usize,
    dir: Dir
}
impl Pos {
    fn step(&self, map: &Map) -> Option<Self> {
        self.dir.step(self.x, self.y, map.height, map.width)
        .map(|(newx, newy)| match map[(newx, newy)] {
            b'#' | b'M' => Self { x: self.x, y: self.y, dir: self.dir.cw() },
            _ => Self { x: newx, y: newy, dir: self.dir.clone() }
        })
    }
    fn cast(&self, map: &Map) -> Option<Self> {
        let mut end = Some(self.clone());
        while end.as_ref().map_or(false, |pos| pos.dir == self.dir) {
            end = end.and_then(|pos| pos.step(&map));
        }
        end
    }
}
impl Index<&Pos> for Map {
    type Output = u8;

    fn index(&self, pos: &Pos) -> &Self::Output {
        &self[(pos.x, pos.y)]
    }
}
impl IndexMut<&Pos> for Map {
    fn index_mut(&mut self, pos: &Pos) -> &mut Self::Output {
        &mut self[(pos.x, pos.y)]
    }
}

pub fn part_one(input: &str) -> Option<u32> {

    let mut map = Map::new(input);

    let idx = map.data.iter()
        .position(|c| *c == b'^')?;

    let mut pos = Pos {
        x: idx % map.width,
        y: idx / map.width,
        dir: Dir::Up
    };
    map[(pos.x, pos.y)] = b'!';
    let mut nvisited = 1;

    while let Some(new) = pos.step(&map) {
        nvisited += (std::mem::replace(&mut map[&new], b'!') != b'!') as u32;
        pos = new;
    }

    Some(nvisited)
}

type EdgeMap = HashMap<Pos, Option<Pos>>;

fn detect_cycle(start: &Pos, edges: &EdgeMap) -> bool {
    let mut curr: Pos = start.clone();
    while let Some(next) = &edges[&curr] {
        if next == start {
            return true;
        }
        curr = next.clone();
    }
    false
}
type Edge = (Pos, Option<Pos>);
fn modify_map(x: usize, y: usize, map: &mut Map, edges: &mut EdgeMap) -> (Vec<Edge>, Vec<Edge>) {
    let mut new_edges = vec![];
    let mut modified_edges = vec![];

    if map[(x, y)] != b'#' {
        // compute new edges
        for ((x, y), dir) in Dir::iter()
                    .filter_map(|dir| dir.step(x, y, map.height, map.width)
                    .filter(|(x, y)| map[(*x, *y)] != b'#')
                    .map(|tup| (tup, dir.ccw()))) {
            let start = Pos { x, y, dir };
            let end = start.cast(&map);
            new_edges.push((start, end));
        }

        let mut u = x-1;
        let newend = Some(Pos {
            x: x-1,
            y,
            dir: Down,
        });
        while u > 0 {
            let newpos = Pos {
                x: u,
                y,
                dir: Right
            };
            if edges.contains_key(&newpos) {
                let oldend = edges.insert(newpos.clone(), newend).unwrap();
                modified_edges.push((newpos, oldend));
                break;
            }
            u -= 1;
        }
    }

    (new_edges, modified_edges)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map = Map::new(input);

    let startpos = {
        let idx = map.data.iter()
            .position(|c| *c == b'^')?;
        Pos {
            x: idx % map.width,
            y: idx / map.width,
            dir: Dir::Up
        }
    };

    let mut path = HashSet::new();
    let mut pos = startpos.clone();

    while let Some(new) = pos.step(&map) {
        path.insert((new.x, new.y));
        pos = new;
    }

    let mut ncycles = 0;

    for (x, y) in path {
        let mut set = HashSet::new();
        map[(x, y)] = b'M';

        let mut pos = startpos.clone();
        'walk: while let Some(new) = pos.step(&map) {
            if set.contains(&pos) {
                ncycles += 1;
                break 'walk;
            } else {
                set.insert(pos.clone());
                pos = new;
            }
        }

        map[(x, y)] = b'.';
    }

    Some(ncycles)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));

        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));

        assert_eq!(result, Some(6));
    }
}
