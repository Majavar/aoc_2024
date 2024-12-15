use std::{collections::VecDeque, io::Read};

fn execute_action(grid: &mut [u8], width: usize, robot: &mut usize, m: u8) {
    let delta = match m {
        b'<' => -1,
        b'>' => 1,
        b'^' => -(width as isize),
        b'v' => width as isize,
        _ => unreachable!(),
    };
    let mut queue = VecDeque::new();
    let mut stack = Vec::new();

    queue.push_back(*robot);
    while let Some(c) = queue.pop_front() {
        if stack.contains(&c) {
            continue;
        };

        let next = (c as isize + delta) as usize;
        match grid[next] {
            b'#' => return,
            b'.' => {
                stack.push(c);
            }
            b'O' => {
                stack.push(c);
                queue.push_back(next);
            }
            b'[' => {
                stack.push(c);
                queue.push_back(next);
                queue.push_back(next + 1);
            }
            b']' => {
                stack.push(c);
                queue.push_back(next);
                queue.push_back(next - 1);
            }
            _ => unreachable!(),
        }
    }

    while let Some(c) = stack.pop() {
        let next = (c as isize + delta) as usize;
        grid[next] = grid[c];
        grid[c] = b'.';
    }

    *robot = (*robot as isize + delta) as usize;
}

fn score(grid: &[u8], width: usize, value: u8) -> usize {
    grid.iter()
        .enumerate()
        .filter(|&(_, b)| *b == value)
        .map(|(i, _)| (i / width, i % width))
        .map(|(y, x)| 100 * y + x)
        .sum()
}

fn find_robot(grid: &[u8]) -> usize {
    grid.iter()
        .enumerate()
        .find(|&(_, b)| *b == b'@')
        .map(|(i, _)| i)
        .unwrap()
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let (grid, moves) = input.split_at(input.find("\n\n").unwrap());

    let (width, _, mut grid) = grid
        .bytes()
        .fold((0, 1, Vec::new()), |(x, y, mut grid), byte| {
            if byte == b'\n' {
                (x, y + 1, grid)
            } else {
                grid.push(byte);
                (if y == 1 { x + 1 } else { x }, y, grid)
            }
        });

    let moves = moves
        .lines()
        .flat_map(|l| l.bytes())
        .collect::<Vec<_>>();

    let mut grid_2 = grid
        .iter()
        .flat_map(|&b| match b {
            b'#' => [b'#', b'#'],
            b'O' => [b'[', b']'],
            b'.' => [b'.', b'.'],
            b'@' => [b'@', b'.'],
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();

    let mut robot = find_robot(&grid);
    let mut robot_2 = find_robot(&grid_2);

    for m in moves {
        execute_action(&mut grid, width, &mut robot, m);
        execute_action(&mut grid_2, 2 * width, &mut robot_2, m);
    }

    println!("Part 1: {}", score(&grid, width, b'O'));
    println!("Part 2: {}", score(&grid_2, 2 * width, b'['));
}
