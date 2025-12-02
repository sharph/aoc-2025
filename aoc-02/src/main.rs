use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut stdin = io::stdin();
    let mut input = String::new();
    stdin.read_to_string(&mut input)?;
    let string: String = input
        .chars()
        .filter(|c| c.is_ascii_digit() || c == &',' || c == &'-')
        .collect();
    let count: u64 = string
        .trim()
        .split(",")
        .filter_map(|range| {
            let mut iter = range.split("-");
            let Some(Ok(from)): Option<Result<u64, _>> = iter.next().map(|n| n.parse()) else {
                return None;
            };
            let Some(Ok(to)): Option<Result<u64, _>> = iter.next().map(|n| n.parse()) else {
                return None;
            };
            Some((from, to))
        })
        .flat_map(|(from, to)| from..to + 1)
        .filter(|n| {
            let s = n.to_string();
            let (a, b) = s.split_at(s.len() / 2);
            a == b
        })
        .sum();
    println!("{}", count);
    Ok(())
}
