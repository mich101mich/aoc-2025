#[allow(unused_imports)]
use crate::utils::*;

pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../../input/day_03.txt");
    // let input = "";

    let result = input
        .lines()
        .map(|row| row.as_bytes())
        .map(|row| {
            let first_digit = *row[..row.len() - 1].iter().max().unwrap();
            let start = row.iter().position(|b| *b == first_digit).unwrap();
            let second_digit = *row[start + 1..].iter().max().unwrap();
            (first_digit - b'0') as usize * 10 + (second_digit - b'0') as usize
        })
        .sum::<usize>();

    result!(result);
}
