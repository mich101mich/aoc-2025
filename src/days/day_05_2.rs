#[allow(unused_imports)]
use crate::utils::*;

pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../../input/day_05.txt");
    // let input = "";

    // let mut grid = _grid(input);

    let mut iter = input.lines();

    let mut ranges = iter
        .by_ref()
        .take_while(|l| !l.is_empty())
        .map(|l| sscanf!(l, "{usize}-{usize}").unwrap())
        .map(|(a, b)| a..=b)
        .to_vec();

    ranges.sort_by_key(|r| *r.start());

    let mut result = 0;
    let mut current_end = 0;
    for r in ranges {
        if current_end >= *r.end() {
            // already fully covered
        } else if current_end >= *r.start() {
            result += *r.end() - current_end;
            current_end = *r.end();
        } else {
            result += r.clone().count();
            current_end = *r.end();
        }
    }

    result!(result);
}
