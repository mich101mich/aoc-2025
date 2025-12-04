#[allow(unused_imports)]
use crate::utils::*;

pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../../input/day_04.txt");
    // let input = "";

    let mut grid = dotted_grid(input, '@');

    let neigh = grid.moore();
    let can_be_removed =
        |grid: &Grid<bool>, p: Point| neigh.get_all_neighbors(p).filter(|o| grid[o]).count() < 4;

    let mut queue = grid
        .pos_iter()
        .filter(|p| can_be_removed(&grid, *p))
        .to_vec();
    let mut removed = queue.len();

    for p in &queue {
        grid[p] = false;
    }

    while let Some(pos) = queue.pop() {
        for other in neigh.get_all_neighbors(pos) {
            if grid[other] && can_be_removed(&grid, other) {
                grid[other] = false;
                queue.push(other);
                removed += 1;
            }
        }
    }

    result!(removed);
}
