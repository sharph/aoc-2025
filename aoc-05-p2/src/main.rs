use std::{collections::HashSet, io};

fn main() {
    let stdin = io::stdin();
    let mut nums: HashSet<(u64, u64)> = HashSet::new();
    for (s, e) in stdin
        .lines()
        .map_while(Result::ok)
        .map_while(|l| if l.trim().is_empty() { None } else { Some(l) })
        .map(|l| {
            let mut i = l.splitn(2, '-').map(|s| s.parse::<u64>());
            (
                i.next().expect("expected number").expect("expected number"),
                i.next().expect("expected number").expect("expected number"),
            )
        })
    {
        nums.insert((s, e));
    }
    while let Some((s, e, es, ee)) = nums
        .iter()
        .copied()
        .flat_map(|(s, e)| nums.iter().copied().map(move |(es, ee)| (s, e, es, ee)))
        .filter(|(s, e, es, ee)| !(s == es && e == ee))
        .find(|(s, e, es, ee)| (s >= es && s <= ee) || (e >= es && e <= ee))
    {
        nums.remove(&(es, ee));
        nums.remove(&(s, e));
        nums.insert((es.min(s), ee.max(e)));
    }
    let fresh: u64 = nums.iter().map(|(s, e)| e - s + 1).sum();
    println!("{}", fresh);
}
