use std::{
    collections::{BTreeMap, BTreeSet},
    io::Read,
};

pub fn part1(mut reader: impl Read) -> usize {
    let mut data = Vec::with_capacity(10_000);
    reader.read_to_end(&mut data).unwrap();

    let connections = parse_input(data.as_slice());

    let mut triplets = BTreeSet::new();

    connections.iter().for_each(|(pc, node_connections)| {
        if pc.starts_with(b"t") {
            get_triple_connections(pc, node_connections, &connections, &mut triplets);
        }
    });

    triplets.len()
}

pub fn part2(mut reader: impl Read) -> String {
    let mut data = Vec::with_capacity(10_000);
    reader.read_to_end(&mut data).unwrap();

    let connections = parse_input(data.as_slice());

    let mut computer_groups = Vec::with_capacity(500);

    connections.iter().for_each(|(pc, node_connections)| {
        get_fully_connected(
            node_connections,
            &connections,
            vec![pc],
            &mut computer_groups,
        );
    });

    computer_groups
        .into_iter()
        .max_by_key(|group| group.len())
        .map(|group| {
            group
                .into_iter()
                .map(|pc| String::from_utf8_lossy(pc))
                .collect::<Vec<_>>()
                .join(",")
        })
        .unwrap()
}

fn parse_input(data: &[u8]) -> BTreeMap<&[u8], Vec<&[u8]>> {
    data.split(|c| *c == b'\n')
        .filter(|line| !line.is_empty())
        .fold(BTreeMap::new(), |mut map, line| {
            let (l, r) = line.split_at(2);

            map.entry(l)
                .and_modify(|connected_to| connected_to.push(&r[1..]))
                .or_insert(vec![&r[1..]]);
            map.entry(&r[1..])
                .and_modify(|connected_to| connected_to.push(l))
                .or_insert(vec![l]);

            map
        })
}

fn get_triple_connections<'a>(
    pc_node: &'a [u8],
    pc_connections: &[&'a [u8]],
    all_connections: &BTreeMap<&'a [u8], Vec<&'a [u8]>>,
    triplets: &mut BTreeSet<[&'a [u8]; 3]>,
) {
    let [head_node, tail @ ..] = pc_connections else {
        return;
    };

    let head_connections = all_connections.get(head_node).unwrap();

    tail.iter()
        .filter(|tail_node| head_connections.contains(tail_node))
        .for_each(|tail_node| {
            let mut nodes = [pc_node, head_node, tail_node];
            nodes.sort();
            triplets.insert(nodes);
        });

    get_triple_connections(pc_node, tail, all_connections, triplets);
}

fn get_fully_connected<'a>(
    pc_connections: &[&'a [u8]],
    all_connections: &BTreeMap<&'a [u8], Vec<&'a [u8]>>,
    mut partial: Vec<&'a [u8]>,
    fully_connected: &mut Vec<Vec<&'a [u8]>>,
) {
    let [head_node, tail @ ..] = pc_connections else {
        partial.sort();
        fully_connected.push(partial);
        return;
    };

    get_fully_connected(tail, all_connections, partial.clone(), fully_connected);

    let head_node_connections = all_connections.get(head_node).unwrap();
    if partial
        .iter()
        .all(|group_node| head_node_connections.contains(group_node))
    {
        partial.push(head_node);
        get_fully_connected(tail, all_connections, partial, fully_connected);
    }
}
