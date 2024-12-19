use std::{env::args, fs::read_to_string, collections::VecDeque};

fn main() {
    let (width, height, limit) = if args().len() == 2 { (71, 71, 1024) } else { (7, 7, 12) };
    let filename = args().nth(1).unwrap_or_else(|| "example".to_string());
    let obstacles = read_to_string(filename)
        .unwrap()
        .lines()
        .map(|l| {
            let mut i = l.split(',').map(|c| c.parse::<usize>().unwrap());
            (i.next().unwrap(), i.next().unwrap())
        })
        .collect::<Vec<_>>();

    let start = (0, 0);
    let end = (width - 1, height - 1);
    let mut visited = vec![vec![u64::MAX; height]; width];
    let mut queue = VecDeque::new();

    queue.push_back((start, 0));
    while let Some(p) = queue.pop_front() {
        let ((x, y), d) = p;
        if visited[x][y] <= d {
            continue;
        }

        visited[x][y] = d;
        if (x, y) == end {
            break;
        }

        if x > 0 {
            let n = (x - 1, y);
            if !obstacles[0..limit].contains(&n) {
                queue.push_back((n, d + 1));
            }
        }
        if x < width - 1 {
            let n = (x + 1, y);
            if !obstacles[0..limit].contains(&n) {
               queue.push_back((n, d + 1));
            }
        }
        if y > 0 {
            let n = (x, y - 1);
            if !obstacles[0..limit].contains(&n) {
                queue.push_back((n, d + 1));
            }
        }
        if y < height - 1 {
            let n = (x, y + 1);
            if !obstacles[0..limit].contains(&n) {
                queue.push_back((n, d + 1));
            }
        }
    }

    println!("Part 1: {}", visited[end.0][end.1]);

    let mut visited = vec![vec![u64::MAX; height]; width];
    let mut queue = VecDeque::new();

    queue.push_back((start, 0));
    let mut last = obstacles.len();

    loop {
        let o = &obstacles[0..last];

        while let Some(p) = queue.pop_front() {
            let ((x, y), d) = p;
            if visited[x][y] != u64::MAX {
                continue;
            }

            visited[x][y] = d;
            if (x, y) == end {
                break;
            }

            if x > 0 {
                let n = (x - 1, y);
                if !o.contains(&n) {
                    queue.push_back((n, d + 1));
                }
            }
            if x < width - 1 {
                let n = (x + 1, y);
                if !o.contains(&n) {
                queue.push_back((n, d + 1));
                }
            }
            if y > 0 {
                let n = (x, y - 1);
                if !o.contains(&n) {
                    queue.push_back((n, d + 1));
                }
            }
            if y < height - 1 {
                let n = (x, y + 1);
                if !o.contains(&n) {
                    queue.push_back((n, d + 1));
                }
            }
        }

        if visited[end.0][end.1] != u64::MAX {
            break;
        }

        last -= 1;
        let (x, y) = obstacles[last];
        if (x > 0 && visited[x - 1][y] != u64::MAX) ||
           (x < width - 1 && visited[x + 1][y] != u64::MAX) ||
           (y > 0 && visited[x][y - 1] != u64::MAX) ||
           (y < height - 1 && visited[x][y + 1] != u64::MAX) {
            queue.push_back((obstacles[last], 0));
        }
    }

    println!("Part 2: {},{}", obstacles[last].0, obstacles[last].1);
}
