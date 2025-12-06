use std::{collections::HashMap, error::Error, io};

struct Grid {
    grid: HashMap<(usize, usize), char>,
    width: usize,
    height: usize,
    default: char,
}

impl Grid {
    fn new(default: char) -> Self {
        Self {
            grid: HashMap::default(),
            width: 0,
            height: 0,
            default,
        }
    }

    fn set(&mut self, x: usize, y: usize, val: char) {
        self.width = self.width.max(x + 1);
        self.height = self.height.max(y + 1);
        self.grid.insert((x, y), val);
    }

    fn set_line(&mut self, y: usize, line: &str) {
        for (x, c) in line.chars().enumerate() {
            self.set(x, y, c);
        }
    }

    fn get_col(&self, x: usize) -> String {
        (0..self.height)
            .map(|y| self.grid.get(&(x, y)).unwrap_or(&self.default))
            .collect()
    }

    fn cols(&self) -> impl Iterator<Item = String> {
        (0..self.width).map(|x| self.get_col(x))
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let lines: Vec<_> = io::stdin()
        .lines()
        .map_while(Result::ok)
        .filter(|l| !l.is_empty())
        .collect();
    let mut g = Grid::new(' ');
    for (i, line) in lines.iter().enumerate() {
        g.set_line(i, line);
    }
    let mut sum: u64 = 0;
    let mut current_op = '+';
    let mut accum: u64 = 0;
    for col in g.cols() {
        let (nums, op) = col.split_at(col.len() - 1);
        current_op = match op {
            "+" => {
                sum += accum;
                accum = 0;
                '+'
            }
            "*" => {
                sum += accum;
                accum = 1;
                '*'
            }
            _ => current_op,
        };
        if nums.chars().any(|c| c.is_ascii_digit())
            && let Ok(num) = nums
                .chars()
                .filter(char::is_ascii_digit)
                .collect::<String>()
                .parse::<u64>()
        {
            match current_op {
                '+' => {
                    accum += num;
                }
                '*' => {
                    accum *= num;
                }
                _ => panic!("invalid op"),
            }
        }
    }
    sum += accum;
    println!("{}", sum);
    Ok(())
}
