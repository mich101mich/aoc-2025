#[allow(unused_imports)]
use crate::utils::*;

pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../../input/day_01.txt");
    //     let input = "L68
    // L30
    // R48
    // L5
    // R60
    // L55
    // L1
    // L99
    // R14
    // L82";

    // let mut grid = _grid(input);

    let mut pos = 50;
    let mut result = 0;
    for (dir, mut step) in input.lines().map(|l| sscanf!(l, "{char}{isize}").unwrap()) {
        result += step / 100;
        step %= 100;
        if dir == 'L' {
            if pos != 0 && step >= pos {
                result += 1;
            }
            pos = (pos - step + 100) % 100;
        } else {
            if pos != 0 && pos + step >= 100 {
                result += 1;
            }
            pos = (pos + step + 100) % 100;
        }
    }

    result!(result);
}
