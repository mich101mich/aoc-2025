#[allow(unused_imports)]
use crate::utils::*;

pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../../input/day_03.txt");
    // let input = "";

    let result = input
        .lines()
        .map(|row| row.as_bytes())
        .map(|mut row| {
            let mut ret = 0;
            for remaining in (0..12).rev() {
                let digit = *row[..row.len() - remaining].iter().max().unwrap();
                let start = row.iter().position(|b| *b == digit).unwrap();
                row = &row[start + 1..];
                ret = ret * 10 + (digit - b'0') as usize;
            }
            ret
        })
        .sum::<usize>();

    result!(result);
}
