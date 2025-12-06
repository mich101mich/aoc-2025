#[allow(unused_imports)]
use crate::utils::*;

pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../../input/day_06.txt");
    // let input = "";

    fn parse_num(num: &[char]) -> usize {
        let mut x = 0;
        for c in num {
            if *c != ' ' {
                x = x * 10 + c.to_digit(10).unwrap() as usize;
            }
        }
        x
    }

    let numbers = char_grid(input).transposed();

    let mut iter = numbers.iter();
    let mut result = 0;
    while let Some(header) = iter.next() {
        let (op, num) = header.split_last().unwrap();
        let mut x = parse_num(num);
        while let Some(row) = iter.next()
            && row.iter().any(|c| *c != ' ')
        {
            if *op == '*' {
                x *= parse_num(row);
            } else {
                x += parse_num(row);
            }
        }
        result += x;
    }

    result!(result);
}
