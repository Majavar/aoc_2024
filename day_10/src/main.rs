use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let lines = input.lines().collect::<Vec<_>>();

    let (width, height) = (lines[0].len() + 2, lines.len() + 2);
    let grid = {
        let mut grid = vec![b'.'; width * height];

        input.lines().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                grid[(y + 1) * width + x + 1] = c as u8;
            });
        });

        grid
    };

    for part in 1..=2 {
        let score: usize = grid.iter().enumerate().filter(|&(_, &v)| v == b'0').map(|(p, _)| {
            let mut visited = vec![p];
            let mut queue = vec![(p, b'0')];

            while let Some((p, v)) = queue.pop() {
                [p-width, p+width, p-1, p+1].into_iter().for_each(|n| {
                    if grid[n] == v + 1 && (part == 2 || !visited.contains(&n)) {
                        visited.push(n);
                        queue.push((n, v + 1));
                    }
                });
            }
            visited.iter().filter(|&&p| grid[p] == b'9').count()
        }).sum();

        println!("Part {}: {}", part, score);

    };
}
