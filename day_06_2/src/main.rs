fn main() {
    let input = include_str!("input");
    let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)];

    let (_, height, obstacles, start, len) = input.bytes().fold(
        (0, 0, Vec::new(), (0, 0), 0),
        |(x, y, mut obstacles, start, len), byte| match byte {
            b'.' => (x + 1, y, obstacles, start, len + 1),
            b'#' => (
                x + 1,
                y,
                {
                    obstacles.push((x, y));
                    obstacles
                },
                start,
                len + 1,
            ),
            b'\n' => (0, y + 1, obstacles, start, len),
            b'^' => (x + 1, y, obstacles, (x, y), len + 1),
            _ => unreachable!(),
        },
    );
    let width = len / height;
    let count = (0..height)
        .flat_map(|y| (0..width).map(move |x| (x, y)))
        .map(|(x, y)| {
            let mut o = obstacles.clone();
            o.push((x, y));
            o
        })
        .filter(|o| is_loop(start, o, &directions))
        .count();

    println!("{}", count);
    
}

fn is_loop(start: (i32, i32), obstacles: &[(i32, i32)], directions: &[(i32, i32)]) -> bool {
    let mut visited = Vec::new();
    let mut dirs = directions.iter().cycle();
    let mut state = (start, dirs.next().unwrap());

    while let Some(s) = match state {
        ((px, py), (0, -1)) => obstacles
            .iter()
            .filter(|(x, y)| *x == px && *y < py)
            .max_by_key(|(_, y)| *y)
            .map(|(x, y)| ((*x, y + 1), dirs.next().unwrap())),
        ((px, py), (1, 0)) => obstacles
            .iter()
            .filter(|(x, y)| *y == py && *x > px)
            .min_by_key(|(x, _)| *x)
            .map(|(x, y)| ((x - 1, *y), dirs.next().unwrap())),
        ((px, py), (0, 1)) => obstacles
            .iter()
            .filter(|(x, y)| *x == px && *y > py)
            .min_by_key(|(_, y)| *y)
            .map(|(x, y)| ((*x, y - 1), dirs.next().unwrap())),
        ((px, py), (-1, 0)) => obstacles
            .iter()
            .filter(|(x, y)| *y == py && *x < px)
            .max_by_key(|(x, _)| *x)
            .map(|(x, y)| ((x + 1, *y), dirs.next().unwrap())),
        _ => unreachable!(),
    } {
        if visited.contains(&s) {
            return true;
        }
        visited.push(s);
        state = s;
    }

    false
}