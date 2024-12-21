
use std::{env::args, fs::read_to_string};

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
    let mut distances = vec![-1; width * height];

    distances[start] = 0;
    let mut pos = start;
    while pos != end {
        let next = [pos - width, pos + width, pos - 1, pos + 1].into_iter().filter_map(|n| {
            if grid[n] != b'#' && distances[n] == -1 {
                Some(n)
            } else {
                None
            }
        }).next().unwrap();
        distances[next] = distances[pos] + 1;
        pos = next;
    }

    let dist = |a: usize, b: usize| (a%width).abs_diff(b%width) + (a/width).abs_diff(b/width);

    for (p, (length, gain)) in [(2, 100), (20, 100)].into_iter().enumerate() {
        let count = (0..distances.len())
            .filter(|p| distances[*p] != -1)
            .flat_map(|p| (0..distances.len()).filter(|q| distances[*q] != -1).map(move |q| (p, q)))
            .filter(|(p, q)| dist(*p, *q) <= length)
            .filter(|(p, q)| distances[*q] - distances[*p] - dist(*p, *q) as isize >= gain)
            .count();

        println!("Part {}: {}", p + 1, count);
    }
}
