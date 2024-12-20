use std::io::Read;

use crate::{maze::Maze, vec2d::Vec2d};

pub fn part1<T: Read>(mut reader: T) -> usize {
    let mut data = Vec::with_capacity(10_000);
    reader.read_to_end(&mut data).unwrap();

    let maze = Maze::parse(data.as_mut_slice(), 1000);

    let (mut maze_tiles_data, _) = maze.calculate_tile_scores();
    let maze_tiles = Vec2d::new(maze_tiles_data.as_mut_slice(), maze.width(), maze.height());

    maze_tiles[maze.end()]
}

pub fn part2(mut reader: impl Read) -> usize {
    let mut data = Vec::with_capacity(10_000);
    reader.read_to_end(&mut data).unwrap();

    let maze = Maze::parse(data.as_mut_slice(), 1000);

    let (_, paths) = maze.calculate_tile_scores();

    paths.len()
}
