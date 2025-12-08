use std::{cmp::Ordering, collections::HashSet, io};

#[derive(PartialEq, PartialOrd, Eq, Hash, Clone, Copy, Debug)]
struct Point3D(i64, i64, i64);

impl Point3D {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self(x, y, z)
    }

    fn dist(&self, other: &Self) -> f64 {
        (((self.0 - other.0).abs().pow(2)
            + (self.1 - other.1).abs().pow(2)
            + (self.2 - other.2).abs().pow(2)) as f64)
            .sqrt()
    }
}

fn main() {
    let stdin = io::stdin();
    let points = stdin
        .lines()
        .map_while(Result::ok)
        .filter_map(|l| {
            let mut split = l
                .trim()
                .splitn(3, ',')
                .map(str::parse)
                .map_while(Result::ok);
            let x = split.next()?;
            let y = split.next()?;
            let z = split.next()?;
            Some(Point3D::new(x, y, z))
        })
        .collect::<HashSet<_>>();
    let mut circuits: Vec<HashSet<Point3D>> = Vec::from_iter(
        points
            .iter()
            .map(|p| HashSet::from_iter([*p].iter().cloned())),
    );
    let mut combos: Vec<_> = points
        .iter()
        .copied()
        .flat_map(|p1| points.iter().copied().map(move |p2| (p1, p2)))
        .filter(|(p1, p2)| p1 != p2 && p2 > p1)
        .collect();
    combos.sort_by(|a, b| {
        a.0.dist(&a.1)
            .partial_cmp(&b.0.dist(&b.1))
            .unwrap_or(Ordering::Equal)
    });
    for (p1, p2) in combos.iter() {
        if circuits.iter().any(|l| l.contains(p1) && l.contains(p2)) {
            continue;
        }
        let mut c_iter = circuits
            .iter_mut()
            .filter(|l| l.contains(p1) || l.contains(p2));
        if let Some(c1) = c_iter.next()
            && let Some(c2) = c_iter.next()
        {
            c1.extend(c2.iter().copied());
            c2.clear();
        }
        if circuits.iter().map(|c| c.len()).max() == Some(points.len()) {
            println!("{}", p1.0 * p2.0);
            break;
        }
    }
}
