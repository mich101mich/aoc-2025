#[allow(unused_imports)]
use crate::utils::*;

pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../../input/day_06.txt");
    // let input = "";

    // let mut grid = _grid(input);

    let rows = input.lines().to_vec();
    let (ops, numbers) = rows.split_last().unwrap();
    let numbers = numbers
        .iter()
        .map(|r| r.split_ascii_whitespace().map(parse_u).to_vec())
        .to_vec();
    let ops = ops
        .split_ascii_whitespace()
        .map(|s| s.chars().next().unwrap())
        .to_vec();

    let mut result = 0;
    for (col, op) in ops.iter().enumerate() {
        result += match *op {
            '*' => numbers.iter().map(|r| r[col]).product::<usize>(),
            '+' => numbers.iter().map(|r| r[col]).sum(),
            _ => panic!("Unknown op: {op}"),
        };
    }

    result!(result);
}
