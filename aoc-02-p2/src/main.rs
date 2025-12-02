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
        .flat_map(|(from, to)| from..=to)
        .filter(|n| {
            let s = n.to_string();
            (1..=s.len() / 2).filter(|n| s.len() % n == 0).any(|n| {
                let (patt, rest) = s.split_at(n);
                (0..s.len() / n - 1)
                    .map(|x| x * n..x * n + n)
                    .all(|r| rest[r] == *patt)
            })
        })
        .sum();
    println!("{}", count);
    Ok(())
}
