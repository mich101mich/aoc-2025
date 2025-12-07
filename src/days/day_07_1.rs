#[allow(unused_imports)]
use crate::utils::*;

pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../../input/day_07.txt");
    // let input = "";

    let mut grid = char_grid(input);

    let start = grid.find('S').unwrap();

    grid[start] = '.';
    let grid = grid.map(|c| *c == '^');

    let mut positions = HashSet::new();
    let mut new_positions = HashSet::new();
    positions.insert(start.x);

    let mut result = 0;
    for y in start.y + 1..grid.h() {
        for x in positions.drain() {
            if grid[y][x] {
                result += 1;
                new_positions.insert(x - 1);
                new_positions.insert(x + 1);
            } else {
                new_positions.insert(x);
            }
        }
        swap!(positions, new_positions);
    }

    result!(result);
}
