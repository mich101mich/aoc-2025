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

    let mut positions = HashMap::new();
    let mut new_positions = HashMap::new();
    positions.insert(start.x, 1);

    for y in start.y + 1..grid.h() {
        for (x, n) in positions.drain() {
            if grid[y][x] {
                *new_positions.entry(x - 1).or_default() += n;
                *new_positions.entry(x + 1).or_default() += n;
            } else {
                *new_positions.entry(x).or_default() += n;
            }
        }
        swap!(positions, new_positions);
    }

    let result = positions.values().sum::<usize>();
    result!(result);
}
