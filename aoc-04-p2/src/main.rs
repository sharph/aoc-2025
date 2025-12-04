use std::collections::HashSet;
use std::error::Error;
use std::io;

#[derive(Default)]
struct Grid(HashSet<(i32, i32)>);

impl Grid {
    fn set(&mut self, x: i32, y: i32) {
        self.0.insert((x, y));
    }

    fn del(&mut self, x: i32, y: i32) -> bool {
        self.0.remove(&(x, y))
    }

    fn find_accessible(&self) -> impl Iterator<Item = &(i32, i32)> {
        self.0.iter().filter(|(x, y)| {
            (-1..=1)
                .flat_map(|dx| (-1..=1).map(move |dy| (dx, dy)))
                .filter(|(dx, dy)| !(*dx == 0 && *dy == 0))
                .filter(|(dx, dy)| self.0.contains(&(x + dx, y + dy)))
                .count()
                < 4
        })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let mut grid = Grid::default();
    for ((x, y), c) in stdin
        .lines()
        .map_while(Result::ok)
        .filter(|l| !l.trim().is_empty())
        .enumerate()
        .flat_map(|(y, chars)| {
            chars
                .trim()
                .chars()
                .enumerate()
                .map(move |(x, c)| ((x, y), c))
                .collect::<Vec<_>>()
        })
    {
        if c == '@' {
            grid.set(x.try_into()?, y.try_into()?);
        }
    }
    let mut count: i32 = 0;
    while grid.find_accessible().count() > 0 {
        let to_del: Vec<_> = grid.find_accessible().copied().collect();
        for (x, y) in to_del.iter() {
            grid.del(*x, *y);
            count += 1;
        }
    }
    println!("{}", count);
    Ok(())
}
