#[allow(unused_imports)]
use crate::utils::*;

pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../../input/day_09.txt");
    //     let input = "7,1
    // 11,1
    // 11,7
    // 9,7
    // 9,5
    // 2,5
    // 2,3
    // 7,3";

    // let mut grid = _grid(input);

    let points = input
        .lines()
        .map(|l| sscanf!(l, "{0},{0}", usize).unwrap())
        .map(|(a, b)| p2(a, b))
        .to_vec();

    let mut result = 1;
    for (i, a) in points.iter().enumerate() {
        for b in points[i + 1..].iter() {
            let dx = a.x.abs_diff(b.x) + 1;
            let dy = a.y.abs_diff(b.y) + 1;
            let area = dx * dy;
            result = result.max(area);
        }
    }

    result!(result);
}
