use std::io;

fn main() {
    let stdin = io::stdin();
    let ranges: Vec<(u64, u64)> = stdin
        .lines()
        .map_while(Result::ok)
        .map_while(|l| if l.trim().is_empty() { None } else { Some(l) })
        .map(|l| {
            let mut i = l.splitn(2, '-').map(|s| s.parse());
            (
                i.next().expect("expected number").expect("expected number"),
                i.next().expect("expected number").expect("expected number"),
            )
        })
        .collect();
    let stdin = io::stdin();
    let count = stdin
        .lines()
        .map_while(Result::ok)
        .filter_map(|s| s.trim().parse::<u64>().ok())
        .filter(|n| ranges.iter().any(|(s, e)| s <= n && n <= e))
        .count();
    println!("{}", count);
}
