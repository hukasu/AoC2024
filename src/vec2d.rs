use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct Vec2d<'a, T> {
    data: &'a mut [T],
    width: usize,
    height: usize,
}

impl<'a, T> Vec2d<'a, T> {
    pub fn new(data: &'a mut [T], width: usize, height: usize) -> Vec2d<'a, T> {
        Self {
            data,
            width,
            height,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

impl<T> Index<(usize, usize)> for Vec2d<'_, T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0 * self.width + index.1]
    }
}

impl<T> IndexMut<(usize, usize)> for Vec2d<'_, T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.0 * self.width + index.1]
    }
}
