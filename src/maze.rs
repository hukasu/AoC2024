use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet, VecDeque},
};

use crate::{direction::Direction, vec2d::Vec2d};

type MazeTiles = Vec<usize>;
type MazeContextQueue = VecDeque<MazeContextQueueItem>;
type MazeContextQueueItem = (
    (usize, usize),
    usize,
    Direction,
    BTreeSet<(usize, usize)>,
    bool,
);
type SidePaths = BTreeMap<(usize, usize), BTreeMap<Direction, (usize, BTreeSet<(usize, usize)>)>>;

pub struct Maze<'a> {
    maze: Vec2d<'a, u8>,
    start: (usize, usize),
    end: (usize, usize),
    turn_cost: usize,
}

struct MazeContext<'a> {
    queue: MazeContextQueue,
    maze_tiles: Vec2d<'a, usize>,
    main_path: BTreeSet<(usize, usize)>,
    side_paths: SidePaths,
}

impl<'a> Maze<'a> {
    pub fn parse(data: &'a mut [u8], turn_cost: usize) -> Maze<'a> {
        let maze = data.split(|c| c == &b'\n').collect::<Vec<_>>();
        let height = maze.len();
        let width = maze[0].len();

        let (start, end) = maze.clone().into_iter().enumerate().fold(
            ((0, 0), (0, 0)),
            |mut coords, (row, line)| {
                line.iter().enumerate().for_each(|(column, tile)| {
                    if *tile == b'S' {
                        coords.0 = (row, column);
                    } else if *tile == b'E' {
                        coords.1 = (row, column);
                    }
                });
                coords
            },
        );

        // +1 to include '\n'
        Self {
            maze: Vec2d::new(data, width + 1, height),
            start,
            end,
            turn_cost,
        }
    }

    pub fn width(&self) -> usize {
        self.maze.width()
    }

    pub fn height(&self) -> usize {
        self.maze.height()
    }

    pub fn start(&self) -> (usize, usize) {
        self.start
    }

    pub fn end(&self) -> (usize, usize) {
        self.end
    }

    pub fn calculate_tile_scores(&self) -> (MazeTiles, BTreeSet<(usize, usize)>) {
        let start = self.start();
        let end = self.end();
        assert_eq!(self.maze[start], b'S');
        assert_eq!(self.maze[end], b'E');

        let mut queue = VecDeque::new();

        let mut start_path = BTreeSet::new();
        start_path.insert(start);
        queue.push_back((start, 0, Direction::East, start_path, true));

        // North, South, East, West
        let mut maze_tiles_data = vec![usize::MAX; self.width() * self.height()];
        let mut maze_tiles =
            Vec2d::new(maze_tiles_data.as_mut_slice(), self.width(), self.height());
        // Start has 0 for East
        maze_tiles[start] = 0;

        let mut context = MazeContext {
            queue,
            maze_tiles,
            main_path: BTreeSet::new(),
            side_paths: BTreeMap::new(),
        };

        while let Some(tile) = context.queue.pop_front() {
            // Step
            self.try_step(&tile, &mut context);

            // Turn right
            let right_tile = (
                tile.0,
                tile.1 + self.turn_cost,
                tile.2.turn_right(),
                tile.3.clone(),
                tile.4,
            );
            self.try_step(&right_tile, &mut context);

            // Turn left
            let left_tile = (
                tile.0,
                tile.1 + self.turn_cost,
                tile.2.turn_left(),
                tile.3.clone(),
                tile.4,
            );
            self.try_step(&left_tile, &mut context);
        }

        context.main_path.insert(end);

        for (side_path_end, side_path) in context.side_paths {
            for (_, (side_path_score, side_path)) in side_path {
                if context.main_path.contains(&side_path_end) {
                    let path_score = context.maze_tiles[side_path_end];
                    if side_path_score == path_score {
                        context.main_path.extend(side_path.iter());
                    }
                }
            }
        }

        let main_path = context.main_path;

        (maze_tiles_data, main_path)
    }

    fn try_step(
        &self,
        (coord, coord_score, direction, path, retry): &MazeContextQueueItem,
        context: &mut MazeContext,
    ) {
        let step = direction.step(*coord);
        if self.maze[step] != b'#' {
            let score = &mut context.maze_tiles[step];
            match (*score).cmp(&(coord_score + 1)) {
                Ordering::Greater => {
                    if step != self.end() {
                        *score = coord_score + 1;
                        let mut step_path = path.clone();
                        step_path.insert(step);
                        context
                            .queue
                            .push_back((step, *score, *direction, step_path, *retry));
                    } else {
                        if *score > (coord_score + 1) {
                            context.main_path = path.clone();
                        } else {
                            context.main_path.extend(path.iter());
                        }
                        context.maze_tiles[step] = coord_score + 1;
                    }
                }
                Ordering::Equal => {
                    let mut step_path = path.clone();
                    step_path.insert(step);
                    context
                        .side_paths
                        .entry(step)
                        .and_modify(|side_path| {
                            if let Some((side_path_score, side_path)) = side_path.get_mut(direction)
                            {
                                match (*side_path_score).cmp(score) {
                                    Ordering::Greater => {
                                        *side_path_score = *score;
                                        *side_path = step_path;
                                    }
                                    Ordering::Equal => {
                                        side_path.extend(path.iter());
                                    }
                                    Ordering::Less => (),
                                }
                            } else {
                                side_path.insert(*direction, (*score, step_path));
                            }
                        })
                        .or_insert(BTreeMap::from_iter([(*direction, (*score, path.clone()))]));
                }
                Ordering::Less => {
                    if *retry && step != self.end() {
                        let mut step_path = path.clone();
                        step_path.insert(step);
                        context.queue.push_back((
                            step,
                            coord_score + 1,
                            *direction,
                            step_path,
                            false,
                        ));
                    }
                }
            }
        }
    }
}
