#[allow(unused_imports)]
use crate::utils::*;

pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../../input/day_12.txt");
    //     let input = "0:
    // ###
    // ##.
    // ##.

    // 1:
    // ###
    // ##.
    // .##

    // 2:
    // .##
    // ###
    // ##.

    // 3:
    // ##.
    // ###
    // ##.

    // 4:
    // ###
    // #..
    // ###

    // 5:
    // ###
    // .#.
    // ###

    // 4x4: 0 0 0 0 2 0
    // 12x5: 1 0 1 0 2 2
    // 12x5: 1 0 1 0 3 2";

    // Assumptions (sorry):
    // - Regions are no wider than 64, meaning we can use 64bit masks

    type Shape = Vec<u64>;

    #[derive(Debug)]
    struct ShapeMeta {
        orientations: Vec<Shape>,
        num_set: u32,
    }

    type Shapes = Vec<ShapeMeta>;

    let blocks = input.split("\n\n").to_vec();
    let (regions, shapes) = blocks.split_last().unwrap();

    fn to_shape(grid: &Grid<bool>) -> Shape {
        let mut ret = vec![0; grid.h()];
        for p in grid.pos_iter() {
            ret[p.y] |= 1 << p.x;
        }
        ret
    }

    fn fits(grid: &[u64], shape: &Shape, x: usize, y: usize) -> bool {
        grid.iter()
            .skip(y)
            .zip(shape)
            .all(|(existing, added)| existing & (added << x) == 0)
    }
    fn add(grid: &mut [u64], shape: &Shape, x: usize, y: usize) {
        for (existing, added) in grid.iter_mut().skip(y).zip(shape) {
            *existing |= added << x;
        }
    }
    fn remove(grid: &mut [u64], shape: &Shape, x: usize, y: usize) {
        for (existing, removed) in grid.iter_mut().skip(y).zip(shape) {
            *existing &= !(removed << x);
        }
    }
    fn count_set(grid: &[u64]) -> u32 {
        grid.iter().map(|r| r.count_ones()).sum()
    }
    #[allow(unused)]
    fn print_grid(grid: &[u64], w: usize) {
        for row in grid {
            let row = format!("{row:064b}");
            for c in row.as_bytes().iter().rev().take(w) {
                print!("{}", *c as char);
            }
            println!();
        }
        println!();
    }

    let shapes: Shapes = shapes
        .iter()
        .map(|b| hashtag_grid(b.split_once('\n').unwrap().1))
        .map(|mut shape| {
            let mut orientations = vec![];
            for _ in 0..4 {
                orientations.push(to_shape(&shape));
                shape.rotate_clockwise();
            }
            shape.reverse();
            for _ in 0..4 {
                orientations.push(to_shape(&shape));
                shape.rotate_clockwise();
            }
            orientations.sort_unstable();
            orientations.dedup();

            let num_set = shape.count() as u32;
            ShapeMeta {
                orientations,
                num_set,
            }
        })
        .to_vec();

    fn solve(
        counts: &mut [usize],
        mut required: u32,
        grid: &mut [u64],
        w: usize,
        shapes: &Shapes,
    ) -> bool {
        if counts.iter().all(|c| *c == 0) {
            return true;
        }
        for y in 0..=grid.len() - 3 {
            let remaining_space = (grid.len() - y) * w;
            let remaining = remaining_space as u32 - count_set(&grid[y..]);
            if remaining < required {
                break;
            }

            for x in 0..=w - 3 {
                'outer: for (i, meta) in shapes.iter().enumerate() {
                    if counts[i] == 0 {
                        continue;
                    }
                    for shape in &meta.orientations {
                        if !fits(grid, shape, x, y) {
                            continue;
                        }
                        add(grid, shape, x, y);
                        counts[i] -= 1;
                        required -= meta.num_set;

                        if solve(counts, required, grid, w, shapes) {
                            return true;
                        }

                        required += meta.num_set;
                        counts[i] += 1;
                        remove(grid, shape, x, y);
                        continue 'outer;
                    }
                }
            }
        }

        false
    }

    let result = regions
        .lines()
        .filter(|region| {
            let (w, h, counts) = sscanf!(region, "{usize}x{usize}: {str}").unwrap();
            let mut counts = counts.split_ascii_whitespace().map(parse_u).to_vec();

            let mut required = counts
                .iter()
                .zip(&shapes)
                .map(|(n, s)| *n as u32 * s.num_set)
                .sum();

            let mut grid = vec![0u64; h];

            // Each shape is 3x3, so the first shape has to be placed within the top left 3x3 corner.
            // Proof: Otherwise, you could just fit another shape into that space, meaning the
            //        current tiling wasn't optimal
            for y in 0..3 {
                for x in 0..3 {
                    if y + 3 > h || x + 3 > w {
                        continue;
                    }
                    for (i, meta) in shapes.iter().enumerate() {
                        if counts[i] == 0 {
                            continue;
                        }
                        for shape in &meta.orientations {
                            add(&mut grid, shape, x, y);
                            counts[i] -= 1;
                            required -= meta.num_set;

                            if solve(&mut counts, required, &mut grid, w, &shapes) {
                                return true;
                            }

                            required += meta.num_set;
                            counts[i] += 1;
                            remove(&mut grid, shape, x, y);
                        }
                    }
                }
            }

            false
        })
        .count();

    result!(result);
}
