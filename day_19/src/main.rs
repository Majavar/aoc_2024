use std::{
    collections::HashMap,
    env::args,
    fs::read_to_string,
};

fn count_possible<'a>(pattern: &'a[u8], towels: &[&[u8]], cache: &mut HashMap<&'a[u8], u64>) -> u64 {
    if pattern.len() == 0 {
        return 1;
    }

    (1..=pattern.len()).filter(|&l| towels.contains(&&pattern[..l])).map(|l| {
        if let Some(&c) = cache.get(&pattern[l..]) {c} else {
            let c = count_possible(&pattern[l..], towels, cache);
            cache.insert(&pattern[l..], c);
            c
        }
    }).sum()
}

fn main() {
    let filename = args().nth(1).unwrap_or_else(|| "example".to_string());
    let input = read_to_string(filename).unwrap();
    let mut lines = input.lines();

    let towels = lines.next().unwrap().split(", ").map(|t| t.as_bytes()).collect::<Vec<_>>();

    let _ = lines.next();
    let patterns = lines.map(|p| p.as_bytes()).collect::<Vec<_>>();

    let mut cache = HashMap::new();
    let pattern_variation = patterns.iter().map(|p| count_possible(p, &towels, &mut cache)).collect::<Vec<_>>();

    println!("Part 1: {}", pattern_variation.iter().filter(|&p| *p != 0).count());
    println!("Part 2: {}", pattern_variation.iter().sum::<u64>());
}
