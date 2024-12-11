pub mod template;

// Use this file to add helper functions and additional modules.

pub struct Map {
    pub data: Vec<u8>,
    width: usize,
    height: usize,
}
impl Map {
    pub fn empty(width: usize, height: usize) -> Self {
        Self {
            data: vec![b'.'; width * height],
            width,
            height
        }
    }

    pub fn clear(&mut self) {
        self.data.fill(b'.');
    }

    pub fn new(input: &str) -> Self {
        Self {
            data: input.lines().flat_map(str::bytes).collect(),
            width: input.lines().next().unwrap().len(),
            height: input.lines().count()
        }
    }

    pub fn step(&self, (x, y): (usize, usize), (dx, dy): (i32, i32)) -> Option<(usize, usize)> {
        let x = x as i32 + dx;
        let y = y as i32 + dy;
        (x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32)
            .then(|| (x as usize, y as usize))
    }

    pub fn pos(&self, idx: usize) -> (usize, usize) {
        (idx % self.width, idx / self.width)
    }
    pub fn idx(&self, (x, y): (usize, usize)) -> usize {
        y * self.width + x
    }

    pub fn width(&self) -> usize { self.width }
    pub fn height(&self) -> usize { self.height }

    pub fn get(&self, (x, y): (usize, usize)) -> Option<&u8> {
        (x < self.width && y < self.height)
            .then(|| &self[(x, y)])
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut u8> {
        (x < self.width && y < self.height)
            .then(|| &mut self[(x, y)])
    }

    pub fn to_str(&self) -> Option<String> {
        use std::str;
        self.data[..]
            .chunks(self.width)
            .map(|slice| str::from_utf8(slice).ok())
            .collect::<Option<Vec<&str>>>()
            .map(|v| v.join("\n"))
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
