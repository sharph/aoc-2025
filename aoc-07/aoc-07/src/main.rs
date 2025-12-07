use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let mut beam_active: Vec<bool> = vec![];
    let mut split_count: u64 = 0;
    for line in stdin.lines().map_while(Result::ok) {
        split_count += beam_active
            .iter()
            .zip(line.trim().chars())
            .filter(|(beam, c)| **beam && c == &'^')
            .count() as u64;
        beam_active = line
            .trim()
            .chars()
            .enumerate()
            .zip(
                line.trim()
                    .chars()
                    .chain(['.'].into_iter())
                    .enumerate()
                    .filter(|(i, _)| *i != 0),
            )
            .zip(['.'].into_iter().chain(line.trim().chars()))
            .map(|(((i, c), (i_ahead, c_ahead)), c_behind)| {
                (
                    (i, c),
                    (i_ahead, c_ahead),
                    (i.wrapping_sub(1), c_behind),
                    (
                        beam_active.get(i.wrapping_sub(1)).copied().unwrap_or(false),
                        beam_active.get(i).copied().unwrap_or(false),
                        beam_active.get(i_ahead).copied().unwrap_or(false),
                    ),
                )
            })
            .map(
                |((i, c), (i_ahead, c_ahead), (i_behind, c_behind), (beam_b, beam, beam_a))| {
                    ((beam || c == 'S')
                        || (c_ahead == '^' && beam_a)
                        || (c_behind == '^' && beam_b))
                        && (c != '^')
                },
            )
            .collect();
    }
    println!("{}", split_count);
    Ok(())
}
