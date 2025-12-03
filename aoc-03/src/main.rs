use std::io;

fn main() {
    let stdin = io::stdin();
    let joltage: u32 = stdin
        .lines()
        .map_while(Result::ok)
        .map(|l| {
            l.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .map(|nums| {
            nums.iter().enumerate().fold(
                ((0, 0), (0, 0)),
                |((a_idx, a_val), (b_idx, b_val)): ((usize, u32), (usize, u32)), (i, n)| {
                    if *n > a_val && i < nums.len() - 1 {
                        ((i, *n), (0, 0))
                    } else if i == a_idx + 1 || *n > b_val {
                        ((a_idx, a_val), (i, *n))
                    } else {
                        ((a_idx, a_val), (b_idx, b_val))
                    }
                },
            )
        })
        .map(|((_, a), (_, b))| a * 10 + b)
        .sum();
    println!("{}", joltage);
}
