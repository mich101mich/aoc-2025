#[allow(unused_imports)]
use crate::utils::*;

pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../../input/day_11.txt");
    // let input = "";

    // let mut grid = _grid(input);

    let devices = input
        .lines()
        .map(|l| sscanf!(l, "{str}: {str}").unwrap())
        .map(|(k, v)| (k, v.split_ascii_whitespace().to_vec()))
        .to_map();

    let mut pos = HashMap::new();
    pos.insert("you", 1);
    let mut next_pos = HashMap::new();

    let mut result = 0;
    while !pos.is_empty() {
        for (p, n) in &pos {
            for &next in devices[p].iter() {
                if next == "out" {
                    result += n;
                } else {
                    *next_pos.entry(next).or_default() += n;
                }
            }
        }
        swap!(pos, next_pos);
        next_pos.clear();
    }

    result!(result);
}
