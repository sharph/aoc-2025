use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let mut beam_active: Vec<u64> = vec![];
    for line in stdin.lines().map_while(Result::ok) {
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
                        beam_active
                            .get(i.wrapping_sub(1))
                            .cloned()
                            .unwrap_or_default(),
                        beam_active.get(i).cloned().unwrap_or_default(),
                        beam_active.get(i_ahead).cloned().unwrap_or_default(),
                    ),
                )
            })
            .map(
                |((i, c), (i_ahead, c_ahead), (i_behind, c_behind), (beam_b, beam, beam_a))| {
                    let mut beam_sum = beam;
                    if c == 'S' {
                        beam_sum += 1;
                    }
                    if c == '^' {
                        return 0;
                    }
                    if c_behind == '^' {
                        beam_sum += beam_b;
                    }
                    if c_ahead == '^' {
                        beam_sum += beam_a;
                    }
                    beam_sum
                },
            )
            .collect();
    }
    println!("{}", beam_active.iter().sum::<u64>());
    Ok(())
}
