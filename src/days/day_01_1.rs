#[allow(unused_imports)]
use crate::utils::*;

pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../../input/day_01.txt");
    // let input = "";

    // let mut grid = _grid(input);

    let mut pos = 50;
    let mut result = 0;
    for (dir, step) in input.lines().map(|l| sscanf!(l, "{char}{isize}").unwrap()) {
        if dir == 'L' {
            pos = (pos - step + 100) % 100;
        } else {
            pos = (pos + step + 100) % 100;
        }
        if pos == 0 {
            result += 1;
        }
    }

    result!(result);
}
