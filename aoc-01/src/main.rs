use std::io;

#[derive(Clone, Copy)]
struct Dial(u32);

impl Dial {
    fn new(init: u32) -> Self {
        Self(init % 100)
    }

    fn left(&mut self, val: u32) {
        self.0 = (self.0 + 100 - (val % 100)) % 100;
    }

    fn right(&mut self, val: u32) {
        self.0 = (self.0 + val) % 100;
    }
}

fn main() {
    let mut dial = Dial::new(50);
    let stdin = io::stdin();
    let password = stdin.lines().fold(0, |c, line| {
        let Ok(line) = line else {
            return c;
        };
        let Some((cmd, val)) = line.split_at_checked(1) else {
            return c;
        };
        let Ok(val): Result<u32, _> = val.parse() else {
            return c;
        };
        match cmd {
            "L" => dial.left(val),
            "R" => dial.right(val),
            _ => {
                return c;
            }
        }
        if dial.0 == 0 { c + 1 } else { c }
    });
    println!("{}", password);
}
