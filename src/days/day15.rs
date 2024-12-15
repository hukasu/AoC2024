use std::io::{BufRead, BufReader, Read};

fn run<T: Read>(reader: T, wide: bool) -> usize {
    let mut buf = BufReader::with_capacity(10_000, reader);
    let mut warehouse = Warehouse::parse(&mut buf, wide);

    let mut moves = Vec::with_capacity(10_000);
    buf.read_to_end(&mut moves).unwrap();

    for robot_move in moves.into_iter().filter(|c| c != &b'\n').map(|c| match c {
        b'v' => RobotMove::Down,
        b'^' => RobotMove::Up,
        b'<' => RobotMove::Left,
        b'>' => RobotMove::Right,
        _ => unreachable!("Not a move"),
    }) {
        warehouse.move_robot(robot_move);
    }

    warehouse.compute_gps()
}

pub fn part1<T: Read>(reader: T) -> usize {
    run(reader, false)
}

pub fn part2(reader: impl Read) -> usize {
    run(reader, true)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RobotMove {
    Up,
    Down,
    Left,
    Right,
}

impl RobotMove {
    fn next(&self, coord: (usize, usize)) -> (usize, usize) {
        match self {
            RobotMove::Up => (coord.0 - 1, coord.1),
            RobotMove::Down => (coord.0 + 1, coord.1),
            RobotMove::Left => (coord.0, coord.1 - 1),
            RobotMove::Right => (coord.0, coord.1 + 1),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WarehouseTile {
    Empty,
    Wall,
    BoxLeft,
    BoxRight,
    Robot,
}

#[derive(Debug)]
struct Warehouse {
    robot: (usize, usize),
    map: Vec<WarehouseTile>,
    dimensions: (usize, usize),
    wide: bool,
}

impl Warehouse {
    fn parse<T: Read>(reader: &mut BufReader<T>, wide: bool) -> Self {
        let multiplier = if wide { 2 } else { 1 };
        reader
            .lines()
            .enumerate()
            .take_while(|(_, line)| !line.as_ref().unwrap().is_empty())
            .fold(
                Warehouse {
                    robot: (0, 0),
                    map: Vec::with_capacity(10_000),
                    dimensions: (0, 0),
                    wide,
                },
                |mut warehouse, (row, line)| {
                    let line = line.unwrap();
                    warehouse.dimensions = ((row + 1), line.len() * multiplier);

                    for (column, c) in line.chars().enumerate() {
                        match c {
                            '#' => {
                                warehouse.map.push(WarehouseTile::Wall);
                                if wide {
                                    warehouse.map.push(WarehouseTile::Wall);
                                }
                            }
                            'O' => {
                                warehouse.map.push(WarehouseTile::BoxLeft);
                                if wide {
                                    warehouse.map.push(WarehouseTile::BoxRight);
                                }
                            }
                            '.' => {
                                warehouse.map.push(WarehouseTile::Empty);
                                if wide {
                                    warehouse.map.push(WarehouseTile::Empty);
                                }
                            }
                            '@' => {
                                warehouse.map.push(WarehouseTile::Robot);
                                if wide {
                                    warehouse.map.push(WarehouseTile::Empty);
                                }
                                warehouse.robot = (row, column * multiplier)
                            }
                            _ => unreachable!("Not a tile"),
                        }
                    }

                    warehouse
                },
            )
    }

    fn compute_gps(&self) -> usize {
        self.map
            .iter()
            .enumerate()
            .map(|(i, tile)| {
                let row = i / self.dimensions.1;
                let column = i % self.dimensions.1;
                match tile {
                    WarehouseTile::Empty
                    | WarehouseTile::Wall
                    | WarehouseTile::Robot
                    | WarehouseTile::BoxRight => 0,
                    WarehouseTile::BoxLeft => row * 100 + column,
                }
            })
            .sum()
    }

    fn move_robot(&mut self, robot_move: RobotMove) {
        let next = robot_move.next(self.robot);
        if self.can_push_box(next, robot_move, true) {
            self.propagate_push(self.robot, robot_move, true);
            self.robot = next;
        }
    }

    fn get_coord(&self, coord: (usize, usize)) -> WarehouseTile {
        self.map[coord.0 * self.dimensions.1 + coord.1]
    }

    fn get_coord_mut(&mut self, coord: (usize, usize)) -> &mut WarehouseTile {
        &mut self.map[coord.0 * self.dimensions.1 + coord.1]
    }

    fn can_push_box(
        &mut self,
        coord: (usize, usize),
        robot_move: RobotMove,
        check_sides: bool,
    ) -> bool {
        match self.get_coord(coord) {
            WarehouseTile::Empty => true,
            WarehouseTile::Wall => false,
            WarehouseTile::BoxLeft => {
                let next = robot_move.next(coord);
                let right = (coord.0, coord.1 + 1);

                if right == next {
                    self.can_push_box(next, robot_move, false)
                } else {
                    self.can_push_box(next, robot_move, true)
                        && if self.wide && check_sides {
                            self.can_push_box(right, robot_move, false)
                        } else {
                            true
                        }
                }
            }
            WarehouseTile::BoxRight => {
                let next = robot_move.next(coord);
                let left = (coord.0, coord.1 - 1);

                if left == next {
                    self.can_push_box(next, robot_move, false)
                } else {
                    self.can_push_box(next, robot_move, true)
                        && if check_sides {
                            self.can_push_box(left, robot_move, false)
                        } else {
                            true
                        }
                }
            }
            _ => unreachable!("Invalid coord for move"),
        }
    }

    fn propagate_push(
        &mut self,
        coord: (usize, usize),
        robot_move: RobotMove,
        propagate_sides: bool,
    ) {
        match self.get_coord(coord) {
            WarehouseTile::Empty => (),
            WarehouseTile::Wall => unreachable!("Wall should not be part of propagation"),
            WarehouseTile::BoxLeft => {
                let next = robot_move.next(coord);
                let right = (coord.0, coord.1 + 1);
                if right == next {
                    self.propagate_push(next, robot_move, false);
                } else {
                    self.propagate_push(next, robot_move, true);
                    if self.wide && propagate_sides {
                        self.propagate_push(right, robot_move, false);
                    }
                }
                *self.get_coord_mut(next) = WarehouseTile::BoxLeft;
                *self.get_coord_mut(coord) = WarehouseTile::Empty;
            }
            WarehouseTile::BoxRight => {
                let next = robot_move.next(coord);
                let left = (coord.0, coord.1 - 1);
                if left == next {
                    self.propagate_push(next, robot_move, false);
                } else {
                    self.propagate_push(next, robot_move, true);
                    if self.wide && propagate_sides {
                        self.propagate_push(left, robot_move, false);
                    }
                }
                *self.get_coord_mut(next) = WarehouseTile::BoxRight;
                *self.get_coord_mut(coord) = WarehouseTile::Empty;
            }
            WarehouseTile::Robot => {
                let next = robot_move.next(coord);
                self.propagate_push(next, robot_move, true);
                *self.get_coord_mut(next) = WarehouseTile::Robot;
                *self.get_coord_mut(coord) = WarehouseTile::Empty;
            }
        };
    }
}
