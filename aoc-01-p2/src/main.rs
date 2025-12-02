use std::io;

#[derive(Clone, Copy)]
struct Dial(u32);

impl Dial {
    fn new(init: u32) -> Self {
        Self(init % 100)
    }

    fn left(&mut self, val: u32) -> u32 {
        let inv = (100 - self.0) % 100;
        let clicks = (inv + val) / 100;
        self.0 = (self.0 + 100 - (val % 100)) % 100;
        if val > 0 { clicks } else { 0 }
    }

    fn right(&mut self, val: u32) -> u32 {
        let clicks = (self.0 + val) / 100;
        self.0 = (self.0 + val) % 100;
        if val > 0 { clicks } else { 0 }
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
        c + match cmd {
            "L" => dial.left(val),
            "R" => dial.right(val),
            _ => 0,
        }
    });
    println!("{}", password);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn boundary_cross() {
        let mut d = Dial(99);
        assert_eq!(d.right(2), 1);
        assert_eq!(d.left(2), 1);
    }

    #[test]
    fn no_boundary_cross() {
        let mut d = Dial(90);
        assert_eq!(d.right(2), 0);
        assert_eq!(d.left(2), 0);
    }

    #[test]
    fn many_boundary_cross() {
        let mut d = Dial(90);
        assert_eq!(d.right(200), 2);
        assert_eq!(d.left(200), 2);
    }

    #[test]
    fn land_on_boundary() {
        let mut d = Dial(90);
        assert_eq!(d.right(210), 3);
        assert_eq!(d.0, 0);
        assert_eq!(d.left(210), 2);
    }
}
