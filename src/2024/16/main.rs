use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Empty,
    Wall,
    Start,
    End,
}

// PartialOrd and Ord don't actually do anything useful, but it's for the sake of BinaryHeaps
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
struct Vec3 {
    x: isize,
    y: isize,
}

fn find_start_and_end(map: &Vec<Vec<Tile>>) -> (Vec3, Vec3) {
    let mut start: Vec3 = Default::default();
    let mut end: Vec3 = Default::default();

    for (y, tiles) in map.iter().enumerate() {
        for (x, tile) in tiles.iter().enumerate() {
            let coord = Vec3 {
                x: x as isize,
                y: y as isize,
            };

            match tile {
                Tile::Start => start = coord,
                Tile::End => end = coord,
                _ => (),
            }
        }
    }

    (start, end)
}

const MOVE_COST: i32 = 1;
const TURN_COST: i32 = 1000;

fn compute_dijkstras(map: &Vec<Vec<Tile>>, start: Vec3) -> Vec<Vec<i32>> {
    let n = map.len();
    let m = map[0].len();

    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
    struct Node {
        score: i32,
        pos: Vec3,
        dir: Vec3,
    }

    impl PartialOrd for Node {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.score.partial_cmp(&other.score)
        }
    }

    impl Ord for Node {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.score.cmp(&other.score)
        }
    }

    let mut min_scores = vec![vec![i32::MAX; m]; n];
    min_scores[start.y as usize][start.x as usize] = 0;

    let mut to_process = BinaryHeap::new();
    to_process.push(Reverse(Node {
        score: 0,
        pos: start,
        dir: Vec3 { x: 1, y: 0 },
    }));

    while !to_process.is_empty() {
        let Node { score, pos, dir } = to_process.pop().unwrap().0;

        let dir_left = Vec3 {
            x: -dir.y,
            y: dir.x,
        };
        let dir_right = Vec3 {
            x: dir.y,
            y: -dir.x,
        };
        let dir_back = Vec3 {
            x: -dir.x,
            y: -dir.y,
        };

        let mut try_move = |dir: Vec3, cost: i32| {
            let prev = pos;
            let pos = Vec3 {
                x: prev.x + dir.x,
                y: prev.y + dir.y,
            };
            let x = pos.x as usize;
            let y = pos.y as usize;
            let score = score + cost;

            if map[y][x] != Tile::Wall {
                if score < min_scores[y][x] {
                    min_scores[y][x] = score;
                    to_process.push(Reverse(Node { score, pos, dir }));
                }
            }
        };

        try_move(dir, MOVE_COST);
        try_move(dir_left, TURN_COST + MOVE_COST);
        try_move(dir_right, TURN_COST + MOVE_COST);
        try_move(dir_back, 2 * TURN_COST + MOVE_COST);
    }

    min_scores
}

/// State used in compute_dijkstras_with_state()
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
struct State {
    pos: Vec3,
    dir: Vec3,
}

/// Because (1,1,EAST) != (1,1,WEST), the optimized dijkstra's algorithm above, although working for
/// computing the minimum cost of the shortest path, will not work for computing what the shortest
/// path itself is (well, it does work, but only for a single path - it will fail if there are
/// multiple shortest paths)
fn compute_dijkstras_with_state(map: &Vec<Vec<Tile>>, start: Vec3) -> HashMap<State, Vec<State>> {
    let start = State {
        pos: start,
        dir: Vec3 { x: 1, y: 0 },
    };

    let mut min_paths: HashMap<State, Vec<State>> = Default::default();
    let mut min_scores: HashMap<State, i32> = Default::default();

    min_paths.insert(start, Vec::new());
    min_scores.insert(start, 0);

    let mut to_process = BinaryHeap::new();
    to_process.push(Reverse((0, start)));

    while !to_process.is_empty() {
        let (score, state) = to_process.pop().unwrap().0;

        let mut next_state = |next: State, cost: i32| {
            let x = next.pos.x as usize;
            let y = next.pos.y as usize;
            let score = score + cost;

            if map[y][x] != Tile::Wall {
                let min_score = min_scores.entry(next).or_insert(i32::MAX);
                let min_path = min_paths.entry(next).or_insert_with(|| Vec::new());

                if score < *min_score {
                    *min_score = score;
                    min_path.clear();
                    to_process.push(Reverse((score, next)));
                }

                if score == *min_score {
                    min_path.push(state);
                }
            }
        };

        let State { pos, dir } = state;

        let state_move = State {
            pos: Vec3 {
                x: pos.x + dir.x,
                y: pos.y + dir.y,
            },
            dir,
        };
        let state_left = State {
            pos,
            dir: Vec3 {
                x: -dir.y,
                y: dir.x,
            },
        };
        let state_right = State {
            pos,
            dir: Vec3 {
                x: dir.y,
                y: -dir.x,
            },
        };

        next_state(state_move, MOVE_COST);
        next_state(state_left, TURN_COST);
        next_state(state_right, TURN_COST);
    }

    min_paths
}

fn part_one(map: &Vec<Vec<Tile>>) {
    let (start, end) = find_start_and_end(map);

    let min_scores = compute_dijkstras(map, start);
    let min_score = min_scores[end.y as usize][end.x as usize];

    println!("{}", min_score);
}

fn part_two(map: &Vec<Vec<Tile>>) {
    let (start, end) = find_start_and_end(map);

    let min_paths = compute_dijkstras_with_state(map, start);
    let min_paths_tiles = {
        let end = State {
            pos: end,
            dir: Vec3 { x: 1, y: 0 },
        };

        let mut tiles = HashSet::new();
        let mut min_path = Vec::new();
        min_path.push(&end);

        while !min_path.is_empty() {
            let state = min_path.pop().unwrap();

            tiles.insert(state.pos);

            for prev in min_paths.get(state).unwrap().iter() {
                min_path.push(prev);
            }
        }

        tiles.len()
    };

    println!("{}", min_paths_tiles);
}

fn main() -> std::io::Result<()> {
    let input = aoc::read_input(2024, 16)?;

    let map: Vec<Vec<Tile>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Tile::Empty,
                    '#' => Tile::Wall,
                    'S' => Tile::Start,
                    'E' => Tile::End,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    part_one(&map);
    part_two(&map);

    Ok(())
}
