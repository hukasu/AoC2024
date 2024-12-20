use std::{
    collections::{BTreeMap, BTreeSet},
    io::Read,
};

use crate::{coord::Coord, maze::Maze, vec2d::Vec2d};

pub fn part1(reader: impl Read) -> usize {
    day_20_internal(reader, 2)
}

pub fn part2(reader: impl Read) -> usize {
    day_20_internal(reader, 20)
}

fn day_20_internal(mut reader: impl Read, cheat_len: usize) -> usize {
    let mut data = Vec::with_capacity(100_000);
    reader.read_to_end(&mut data).unwrap();

    let maze = Maze::parse(data.as_mut_slice(), 0);

    let (mut tile_cost_data, main_path) = maze.calculate_tile_scores();
    let tile_cost = Vec2d::new(tile_cost_data.as_mut_slice(), maze.width(), maze.height());

    let cheats = cheat(&main_path, &tile_cost, cheat_len);

    cheats
        .into_iter()
        .map(|(k, v)| if k >= 100 { v } else { 0 })
        .sum()
}

fn cheat(
    main_path: &BTreeSet<Coord>,
    tile_cost: &Vec2d<usize>,
    cheat_len: usize,
) -> BTreeMap<usize, usize> {
    let mut map = BTreeMap::new();

    let paralelism = std::thread::available_parallelism().unwrap().get();

    let main_path = Vec::from_iter(main_path);
    let iters = main_path.chunks((main_path.len() / paralelism) + 1);
    std::thread::scope(|scope| {
        let (sender, receiver) = std::sync::mpsc::channel();

        for iter_ref in iters {
            let sender_clone = sender.clone();

            scope.spawn(move || {
                let sender = sender_clone;

                for tile in iter_ref {
                    sender
                        .send(find_reachable(**tile, tile_cost, cheat_len))
                        .unwrap();
                }
            });
        }

        std::mem::drop(sender);

        for tile_reachables in receiver {
            for (_, cost_cut) in tile_reachables {
                map.entry(cost_cut)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
        }
    });

    map
}

fn find_reachable(
    from: Coord,
    tile_cost: &Vec2d<usize>,
    cheat_len: usize,
) -> BTreeSet<(Coord, usize)> {
    assert_ne!(tile_cost[from], usize::MAX);

    let mut set = BTreeSet::new();

    let bounds = Coord::new(tile_cost.height(), tile_cost.width());

    let cheat_len_i = isize::try_from(cheat_len).unwrap();

    for (row_offset, column_offset) in ((-cheat_len_i)..=(cheat_len_i)).flat_map(|row_offset| {
        let remaining_for_column = isize::try_from(
            cheat_len_i
                .abs_diff(row_offset)
                .min(cheat_len_i.abs_diff(-row_offset)),
        )
        .unwrap();
        ((-remaining_for_column)..=(remaining_for_column))
            .map(move |column_offset| (row_offset, column_offset))
    }) {
        let Some(no_clip) = from.row.checked_add_signed(row_offset).and_then(|row| {
            from.column
                .checked_add_signed(column_offset)
                .map(|column| Coord::new(row, column))
                .filter(|coord| coord.row < bounds.row && coord.column < bounds.column)
        }) else {
            continue;
        };
        if from != no_clip
            && tile_cost[from] < tile_cost[no_clip]
            && tile_cost[no_clip] != usize::MAX
        {
            let clipped = row_offset.unsigned_abs() + column_offset.unsigned_abs();
            let short_cut = tile_cost[no_clip] - tile_cost[from] - clipped;
            if short_cut > 0 {
                set.insert((no_clip, short_cut));
            }
        }
    }

    set
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_reachable_test() {
        #[rustfmt::skip]
        let mut data = vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9,
            usize::MAX, usize::MAX, usize::MAX, usize::MAX, usize::MAX, usize::MAX, usize::MAX, usize::MAX, usize::MAX, 10,
            20, 19, 18, 17, 16, 15, 14, 13, 12, 11,
            21, usize::MAX, usize::MAX, usize::MAX, usize::MAX, usize::MAX, usize::MAX, usize::MAX, usize::MAX, usize::MAX,
            22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
            usize::MAX, usize::MAX, usize::MAX, usize::MAX, usize::MAX, usize::MAX, usize::MAX, usize::MAX, usize::MAX, 32,
            42, 41, 40, 39, 38, 37, 36, 35, 34, 33,
            43, usize::MAX, usize::MAX, usize::MAX, usize::MAX, usize::MAX, usize::MAX, usize::MAX, usize::MAX, usize::MAX,
            44, 45, 46, 47, 48, 49, 50, 51, 52, 53,
            usize::MAX, usize::MAX, usize::MAX, usize::MAX, usize::MAX, usize::MAX, usize::MAX, usize::MAX, usize::MAX, 54,
        ];
        let tile_cost = Vec2d::new(&mut data, 10, 10);
        let reachable = find_reachable(Coord::default(), &tile_cost, 20);

        #[rustfmt::skip]
        let expect = BTreeSet::from_iter(
            [
                (Coord::new(2, 0), 18), (Coord::new(2, 1), 16), (Coord::new(2, 2), 14), (Coord::new(2, 3), 12), (Coord::new(2, 4), 10), (Coord::new(2, 5), 8), (Coord::new(2, 6), 6), (Coord::new(2, 7), 4), (Coord::new(2, 8), 2),
                (Coord::new(3, 0), 18),
                (Coord::new(4, 0), 18), (Coord::new(4, 1), 18), (Coord::new(4, 2), 18), (Coord::new(4, 3), 18), (Coord::new(4, 4), 18), (Coord::new(4, 5), 18), (Coord::new(4, 6), 18), (Coord::new(4, 7), 18), (Coord::new(4, 8), 18), (Coord::new(4, 9), 18),
                                                                                                                                                                                                                                        (Coord::new(5, 9), 18),
                (Coord::new(6, 0), 36), (Coord::new(6, 1), 34), (Coord::new(6, 2), 32), (Coord::new(6, 3), 30), (Coord::new(6, 4), 28), (Coord::new(6, 5), 26), (Coord::new(6, 6), 24), (Coord::new(6, 7), 22), (Coord::new(6, 8), 20), (Coord::new(6, 9), 18),
                (Coord::new(7, 0), 36), 
                (Coord::new(8, 0), 36), (Coord::new(8, 1), 36), (Coord::new(8, 2), 36), (Coord::new(8, 3), 36), (Coord::new(8, 4), 36), (Coord::new(8, 5), 36), (Coord::new(8, 6), 36), (Coord::new(8, 7), 36), (Coord::new(8, 8), 36), (Coord::new(8, 9), 36),
                                                                                                                                                                                                                                        (Coord::new(9, 9), 36),
                
            ]
        );

        assert_eq!(reachable, expect);
    }
}
