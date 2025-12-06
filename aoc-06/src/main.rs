use std::io;

fn main() {
    let mut lines: Vec<_> = io::stdin()
        .lines()
        .map_while(Result::ok)
        .filter(|l| !l.is_empty())
        .collect();
    let operators: Vec<String> = lines
        .pop()
        .expect("expected input")
        .trim()
        .split(" ")
        .filter(|v| !v.is_empty())
        .map(|v| v.to_string())
        .collect();
    let mut values: Vec<u64> = operators
        .iter()
        .map(|o| match o.as_str() {
            "*" => 1,
            "+" => 0,
            _ => panic!("unknown operator"),
        })
        .collect();
    for row in lines.iter().map(|l| {
        l.trim()
            .split(" ")
            .filter(|v| !v.is_empty())
            .map_while(|v| v.parse::<u64>().ok())
    }) {
        for ((val, operator), operand) in values.iter_mut().zip(operators.iter()).zip(row) {
            match operator.as_str() {
                "*" => {
                    *val *= operand;
                }
                "+" => {
                    *val += operand;
                }
                _ => panic!("unknown operator"),
            }
        }
    }
    let sum: u64 = values.iter().sum();
    println!("{}", sum);
}
