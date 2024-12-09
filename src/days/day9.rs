use std::io::{BufReader, Read};

fn build_disk_map(reader: impl Read) -> impl Iterator<Item = DiskMap> {
    let e: fn(u64, u64) -> DiskMap = DiskMap::File;
    reader.bytes().enumerate().scan(e, |e, byte| {
        let (i, byte) = (byte.0, byte.1.unwrap());
        let d = e(u64::try_from(i).unwrap() / 2, u64::from(byte - b'0'));
        *e = d.next();
        Some(d)
    })
}

pub fn part1(reader: impl Read) -> u64 {
    let buf = BufReader::with_capacity(100_000, reader);

    let mut disk_map_indexes = build_disk_map(buf)
        .flat_map(|disk_map| match disk_map {
            // Adding one here because 0 is used as free space
            DiskMap::File(id, len) => vec![id + 1; usize::try_from(len).unwrap()],
            DiskMap::Free(_, len) => vec![0; usize::try_from(len).unwrap()],
        })
        .collect::<Vec<_>>();

    let mut iter = disk_map_indexes.iter_mut().enumerate();
    'outer: loop {
        let (back_i, back) = loop {
            match iter.next_back() {
                Some((_, 0)) => (),
                Some((i, back)) => break (i, back),
                None => break 'outer,
            }
        };
        let (front_i, front) = loop {
            match iter.next() {
                Some((i, front)) if *front == 0 => break (i, front),
                Some(_) => (),
                None => break 'outer,
            }
        };
        if front_i >= back_i {
            break;
        } else {
            *front = *back;
            *back = 0;
        }
    }

    disk_map_indexes
        .iter()
        .enumerate()
        .map(|(i, id)| u64::try_from(i).unwrap() * id.saturating_sub(1))
        .sum()
}

pub fn part2(reader: impl Read) -> u64 {
    let buf = BufReader::with_capacity(100_000, reader);

    let mut disk_map_indexes = build_disk_map(buf)
        .filter(|disk_map| !matches!(disk_map, DiskMap::Free(_, 0)))
        .collect::<Vec<_>>();

    let mut front_i = 0;
    let mut back_i = disk_map_indexes.len() - 1;

    'outer: while front_i <= back_i {
        let back_size = loop {
            match disk_map_indexes.get(back_i) {
                Some(DiskMap::Free(_, _)) => back_i -= 1,
                Some(DiskMap::File(_, size)) => break *size,
                None => break 'outer,
            }
        };
        let front_size = loop {
            if front_i >= back_i {
                back_i -= 1;
                front_i = 1;
                continue 'outer;
            }
            match disk_map_indexes.get(front_i) {
                Some(DiskMap::File(_, _)) => front_i += 1,
                Some(DiskMap::Free(_, size)) => {
                    if *size >= back_size {
                        break *size;
                    } else {
                        front_i += 1;
                    }
                }
                None => {
                    back_i -= 1;
                    front_i = 0;
                    continue 'outer;
                }
            }
        };

        move_file_to_free(
            &mut disk_map_indexes,
            front_i,
            back_i,
            front_size,
            back_size,
        );
        front_i = 0;
    }

    disk_map_indexes
        .iter()
        .flat_map(|diskmap| match diskmap {
            DiskMap::File(id, len) => vec![*id; usize::try_from(*len).unwrap()],
            DiskMap::Free(_, len) => vec![0; usize::try_from(*len).unwrap()],
        })
        .enumerate()
        .map(|(i, id)| u64::try_from(i).unwrap() * id)
        .sum()
}

fn move_file_to_free(
    disk_map: &mut Vec<DiskMap>,
    dst: usize,
    src: usize,
    front_size: u64,
    back_size: u64,
) {
    let back = disk_map.remove(src);
    let additional_size = if let Some(DiskMap::Free(_, size2)) = disk_map.get(src) {
        *size2
    } else {
        0
    };
    if additional_size > 0 {
        disk_map.remove(src);
    }
    if let Some(DiskMap::Free(_, size)) = disk_map.get_mut(src) {
        *size += back_size + additional_size;
    } else {
        disk_map.insert(src, DiskMap::Free(0, back_size + additional_size));
    };

    if front_size == back_size {
        if let Some(disk_map) = disk_map.get_mut(dst) {
            *disk_map = back
        } else {
            panic!("Indexes must have front index");
        };
    } else {
        if let Some(DiskMap::Free(_, size)) = disk_map.get_mut(dst) {
            *size -= back_size;
        } else {
            panic!("Front must be Free");
        }
        disk_map.insert(dst, back);
    }
}

#[derive(Debug, Clone, PartialEq)]
enum DiskMap {
    File(u64, u64),
    #[allow(dead_code)]
    Free(u64, u64),
}

impl DiskMap {
    pub fn next(&self) -> fn(u64, u64) -> Self {
        match self {
            Self::File(_, _) => Self::Free,
            Self::Free(_, _) => Self::File,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move() {
        let mut test = vec![
            DiskMap::File(0, 1),
            DiskMap::Free(0, 1),
            DiskMap::File(1, 1),
            DiskMap::File(2, 1),
        ];
        move_file_to_free(&mut test, 1, 3, 1, 1);
        assert_eq!(
            test,
            vec![
                DiskMap::File(0, 1),
                DiskMap::File(2, 1),
                DiskMap::File(1, 1),
                DiskMap::Free(0, 1),
            ]
        );

        let mut test = vec![
            DiskMap::File(0, 1),
            DiskMap::Free(0, 2),
            DiskMap::File(1, 1),
            DiskMap::File(2, 1),
        ];
        move_file_to_free(&mut test, 1, 3, 2, 1);
        assert_eq!(
            test,
            vec![
                DiskMap::File(0, 1),
                DiskMap::File(2, 1),
                DiskMap::Free(0, 1),
                DiskMap::File(1, 1),
                DiskMap::Free(0, 1),
            ]
        );

        let mut test = vec![
            DiskMap::File(0, 1),
            DiskMap::Free(0, 1),
            DiskMap::File(1, 1),
            DiskMap::Free(1, 1),
            DiskMap::File(2, 1),
        ];
        move_file_to_free(&mut test, 1, 4, 1, 1);
        assert_eq!(
            test,
            vec![
                DiskMap::File(0, 1),
                DiskMap::File(2, 1),
                DiskMap::File(1, 1),
                DiskMap::Free(1, 2),
            ]
        );

        let mut test = vec![
            DiskMap::File(0, 1),
            DiskMap::Free(0, 2),
            DiskMap::File(1, 1),
            DiskMap::Free(1, 1),
            DiskMap::File(2, 1),
        ];
        move_file_to_free(&mut test, 1, 4, 2, 1);
        assert_eq!(
            test,
            vec![
                DiskMap::File(0, 1),
                DiskMap::File(2, 1),
                DiskMap::Free(0, 1),
                DiskMap::File(1, 1),
                DiskMap::Free(1, 2),
            ]
        );

        let mut test = vec![
            DiskMap::File(0, 1),
            DiskMap::Free(0, 2),
            DiskMap::File(1, 1),
            DiskMap::Free(1, 2),
            DiskMap::File(2, 1),
            DiskMap::Free(2, 2),
        ];
        move_file_to_free(&mut test, 1, 4, 2, 1);
        assert_eq!(
            test,
            vec![
                DiskMap::File(0, 1),
                DiskMap::File(2, 1),
                DiskMap::Free(0, 1),
                DiskMap::File(1, 1),
                DiskMap::Free(1, 5),
            ]
        );

        let mut test = vec![
            DiskMap::File(0, 1),
            DiskMap::Free(0, 1),
            DiskMap::File(1, 1),
            DiskMap::File(2, 1),
            DiskMap::File(3, 1),
        ];
        move_file_to_free(&mut test, 1, 3, 1, 1);
        assert_eq!(
            test,
            vec![
                DiskMap::File(0, 1),
                DiskMap::File(2, 1),
                DiskMap::File(1, 1),
                DiskMap::Free(0, 1),
                DiskMap::File(3, 1),
            ]
        );

        let mut test = vec![
            DiskMap::File(0, 1),
            DiskMap::Free(0, 2),
            DiskMap::File(1, 1),
            DiskMap::File(2, 1),
            DiskMap::File(3, 1),
        ];
        move_file_to_free(&mut test, 1, 3, 2, 1);
        assert_eq!(
            test,
            vec![
                DiskMap::File(0, 1),
                DiskMap::File(2, 1),
                DiskMap::Free(0, 1),
                DiskMap::File(1, 1),
                DiskMap::Free(0, 1),
                DiskMap::File(3, 1),
            ]
        );
    }
}
