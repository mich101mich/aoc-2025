#[allow(unused_imports)]
use crate::utils::*;

pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../../input/day_05.txt");
    // let input = "";

    // let mut grid = _grid(input);

    let mut iter = input.lines();

    let ranges = iter
        .by_ref()
        .take_while(|l| !l.is_empty())
        .map(|l| sscanf!(l, "{usize}-{usize}").unwrap())
        .map(|(a, b)| a..=b)
        .to_vec();

    let result = iter
        .map(parse_u)
        .filter(|x| ranges.iter().any(|r| r.contains(x)))
        .count();

    result!(result);
}
