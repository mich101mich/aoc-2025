#[allow(unused_imports)]
use crate::utils::*;

pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../../input/day_04.txt");
    // let input = "";

    let grid = dotted_grid(input, '@');

    let neigh = grid.moore();

    let result = grid
        .pos_iter()
        .filter(|p| neigh.get_all_neighbors(*p).filter(|o| grid[o]).count() < 4)
        .count();

    result!(result);
}
