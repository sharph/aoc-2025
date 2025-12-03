use std::cmp::Ordering;
use std::io;

const BATTERIES: usize = 12;

fn main() {
    let stdin = io::stdin();
    let joltage: u64 = stdin
        .lines()
        .map_while(Result::ok)
        .map(|l| {
            l.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .map(|nums| {
            let mut batteries: Vec<u64> = Vec::new();
            let mut idx = 0;
            while batteries.len() < BATTERIES {
                let Some((i, v)) = nums
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| *i >= idx && *i <= nums.len() - BATTERIES + batteries.len())
                    .max_by(|(a_i, a), (b_i, b)| match (a.cmp(b), b_i.cmp(a_i)) {
                        (Ordering::Less, _) => Ordering::Less,
                        (Ordering::Greater, _) => Ordering::Greater,
                        (Ordering::Equal, o) => o,
                    })
                else {
                    break;
                };
                idx = i + 1;
                batteries.push(*v as u64);
            }
            batteries
                .iter()
                .rev()
                .enumerate()
                .map(|(p, n)| 10_u64.pow(p as u32) * n)
                .sum::<u64>()
        })
        .sum();
    println!("{}", joltage);
}
