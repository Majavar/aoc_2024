use std::{
    cmp::{Ord, Reverse},
    collections::BinaryHeap,
    env::args,
    fs::read_to_string,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct State {
    cost: u64,
    position: usize,
    direction: (isize, isize),
    previous: (usize, (isize, isize)),
}

impl State {
    pub fn new(start: usize) -> Self {
        Self {
            cost: 0,
            position: start,
            direction: (1, 0),
            previous: (start, (1, 0)),
        }
    }

    pub fn index(&self) -> usize {
        index(self.position, self.direction)
    }

    pub fn next(&self, width: usize) -> [State; 3] {
        let d = match self.direction {
            (_, 0) => [(0, 1), (0, -1)],
            (0, _) => [(1, 0), (-1, 0)],
            _ => unreachable!(),
        };
        let p = self.position as isize + self.direction.0 + self.direction.1 * width as isize;
        [
            State {
                cost: self.cost + 1000,
                position: self.position,
                direction: d[0],
                previous: (self.position, self.direction),
            },
            State {
                cost: self.cost + 1000,
                position: self.position,
                direction: d[1],
                previous: (self.position, self.direction),
            },
            State {
                cost: self.cost + 1,
                position: p as usize,
                direction: self.direction,
                previous: (self.position, self.direction),
            },
        ]
    }
}

#[derive(Debug, Clone)]
pub struct Visit {
    cost: u64,
    previous: Vec<(usize, (isize, isize))>,
}

pub fn index(position: usize, direction: (isize, isize)) -> usize {
    match direction {
        (1, 0) => position << 2,
        (-1, 0) => position << 2 | 1,
        (0, 1) => position << 2 | 2,
        (0, -1) => position << 2 | 3,
        _ => unreachable!(),
    }
}

fn main() {
    let filename = args().nth(1).unwrap_or_else(|| "example".to_string());
    let (grid, height, start, end) = read_to_string(filename).unwrap().bytes().fold(
        (Vec::new(), 0, 0, 0),
        |(mut grid, height, start, end), byte| match byte {
            b'.' | b'#' => {
                grid.push(byte);
                (grid, height, start, end)
            }
            b'S' => {
                let s = grid.len();
                grid.push(b'.');
                (grid, height, s, end)
            }
            b'E' => {
                let e = grid.len();
                grid.push(b'.');
                (grid, height, start, e)
            }
            b'\n' => (grid, height + 1, start, end),
            _ => unreachable!(),
        },
    );
    let width = grid.len() / height;

    let mut heap = BinaryHeap::new();
    heap.push(Reverse(State::new(start)));
    let mut visited: Vec<Option<Visit>> = vec![None; width * height * 4];

    while let Some(Reverse(state)) = heap.pop() {
        let index = state.index();
        if let Some(visit) = &mut visited[index] {
            if visit.cost == state.cost {
                visit.previous.push(state.previous);
            }
            continue;
        } else {
            visited[index] = Some(Visit {
                cost: state.cost,
                previous: vec![state.previous],
            });

            for next in state.next(width) {
                if grid[next.position] != b'#' {
                    heap.push(Reverse(next));
                }
            }
        }
    }

    let best = visited[(end << 2)..=(end << 2 | 3)]
        .iter()
        .map(|visit| visit.as_ref().unwrap().cost)
        .min()
        .unwrap();
    println!("Part 1: {}", best);

    let mut heap = ((end << 2)..=(end << 2 | 3))
        .filter(|&i| visited[i].as_ref().unwrap().cost == best)
        .collect::<Vec<_>>();
    let mut positions = Vec::new();

    while let Some(i) = heap.pop() {
        let pos = visited[i].as_ref().unwrap();
        heap.extend(
            pos.previous
                .iter()
                .map(|&(p, d)| index(p, d))
                .filter(|&p| p != i),
        );

        if !positions.contains(&(i >> 2)) {
            positions.push(i >> 2);
        }
    }

    println!("Part 2: {}", positions.len());
}
