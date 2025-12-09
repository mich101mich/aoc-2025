use std::ops::RangeInclusive;

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

    let points = input
        .lines()
        .map(|l| sscanf!(l, "{0},{0}", isize).unwrap())
        .map(|(a, b)| p2(a, b))
        .to_vec();

    // {
    //     let points2 = points.iter().map(|p| p / 500).to_vec();
    //     let w = points2.iter().map(|p| p.x).max().unwrap() + 1;
    //     let h = points2.iter().map(|p| p.y).max().unwrap() + 1;
    //     let mut grid = Grid::<char>::new_clone(p2(w, h), '.');
    //     for w in points2.windows(2) {
    //         let (a, b) = (w[0], w[1]);
    //         if a.x != b.x {
    //             for x in a.x.min(b.x)..a.x.max(b.x) {
    //                 grid[p2(x, a.y)] = 'X';
    //             }
    //         } else {
    //             for y in a.y.min(b.y)..a.y.max(b.y) {
    //                 grid[p2(a.x, y)] = 'X';
    //             }
    //         }
    //     }
    //     for p in points2 {
    //         grid[p] = '#';
    //     }
    //     grid.print();
    // }

    #[derive(Default, Clone)]
    struct Row {
        /// red tiles on this row
        x: Vec<isize>,
        /// places that are inside
        inside: Vec<RangeInclusive<isize>>,
    }
    let mut by_y: BTreeMap<isize, Row> = BTreeMap::new();

    let wrapper = [*points.last().unwrap(), *points.first().unwrap()];
    let lines = points
        .windows(2)
        .chain(std::iter::once(wrapper.as_slice()))
        .to_vec();

    let mut in_dir = HashMap::new();
    let mut out_dir = HashMap::new();

    for w in &lines {
        let (a, b) = (w[0], w[1]);
        let a_to_b = Dir::from_difference(a, b);
        out_dir.insert(a, a_to_b);
        in_dir.insert(b, a_to_b.opposite());

        if a_to_b.is_horizontal() {
            by_y.entry(a.y)
                .or_default()
                .inside
                .push(a.x.min(b.x)..=a.x.max(b.x));
        }
    }

    for p in &points {
        let row = by_y.entry(p.y).or_default();
        row.x.push(p.x);
    }

    let mut vertical_lines: Vec<isize> = vec![];
    for (&y, row) in by_y.iter_mut() {
        // println!();
        // pv!(y, row.x, row.inside);
        // step 1: merge adjacent ranges
        let mut hori_lines = vec![];
        {
            row.inside.sort_unstable_by_key(|r| *r.start());
            let (first, rest) = row.inside.split_first().unwrap();
            let mut prev = first.clone();
            for r in rest {
                if r.start() == prev.end() {
                    prev = *prev.start()..=*r.end();
                } else {
                    hori_lines.push(prev);
                    prev = r.clone();
                }
            }
            hori_lines.push(prev);
        }
        // pv!(hori_lines);

        // step 2: remove vertical lines that end here
        vertical_lines.retain(|x| !hori_lines.iter().any(|r| r.start() == x || r.end() == x));
        // pv!(vertical_lines);

        // sanity check
        let intersecting = vertical_lines
            .iter()
            .find(|x| hori_lines.iter().any(|r| r.contains(x)));
        assert_eq!(intersecting, None, "In row {y}");

        // step 3: separate lines that flip inside-outside with standalone ranges
        row.inside.clear();
        let mut new_vertical_lines = vec![];
        hori_lines.retain(|r| {
            let (a, b) = (p2(*r.start(), y), p2(*r.end(), y));

            // step 2.5: Find new vertical lines
            if in_dir[&a] == Dir::Down || out_dir[&a] == Dir::Down {
                new_vertical_lines.push(a.x);
            }
            if in_dir[&b] == Dir::Down || out_dir[&b] == Dir::Down {
                new_vertical_lines.push(b.x);
            }
            // end of step 2.5

            let is_standalone = if in_dir[&a].is_horizontal() {
                out_dir[&a] == in_dir[&b]
            } else {
                in_dir[&a] == out_dir[&b]
            };
            if is_standalone {
                //               |   |
                //  +---+   or   +---+
                //  |   |
                // standalone inside range
                row.inside.push(r.clone());
            }
            //      |        |
            //  +---+   or   +---+
            //  |                |
            // flipping range
            !is_standalone
        });
        // pv!(row.inside, hori_lines);

        // step 4: find all points where we change between inside and outside
        let mut flips = hori_lines.clone();
        flips.extend(vertical_lines.iter().map(|&x| x..=x));
        flips.sort_unstable_by_key(|r| *r.start());
        // pv!(flips);

        // step 5: find full ranges that are inside
        let mut range_start = None;
        for x in flips {
            if let Some(start) = range_start.take() {
                row.inside.push(start..=*x.end());
            } else {
                range_start = Some(*x.start());
            }
        }
        assert!(range_start.is_none());

        row.inside.sort_unstable_by_key(|r| *r.start());
        // pv!(row.inside);

        // sanity check
        let adjacent = row
            .inside
            .windows(2)
            .find(|w| *w[0].end() + 1 == *w[1].start());
        assert_eq!(adjacent, None);

        // step 6: add new vertical lines
        vertical_lines.extend_from_slice(&new_vertical_lines);
        // pv!(vertical_lines);
    }

    /// Returns if all points in ax..=bx are inside the loop set by the flips
    fn all_inside(ax: isize, bx: isize, inside: &[RangeInclusive<isize>]) -> bool {
        for r in inside {
            let (has_a, has_b) = (r.contains(&ax), r.contains(&bx));
            if has_a && has_b {
                return true; // fully contained
            } else if has_a || has_b {
                return false; // only one side is contained
            }
        }
        false // not in any inside range
    }

    let relevant_y = by_y.into_iter().to_vec();
    let mut result = 0;
    for (i, (ty, top_row)) in relevant_y.iter().enumerate() {
        for (j, (by, bottom_row)) in relevant_y[i + 1..].iter().enumerate() {
            let dy = by - ty + 1;
            for &tx in &top_row.x {
                for &bx in &bottom_row.x {
                    // println!();
                    // pv!(tx, ty, bx, by);
                    let (ax, bx) = (tx.min(bx), tx.max(bx));
                    let mut valid = true;
                    'outer: for r in &relevant_y[i..=i + 1 + j] {
                        if !all_inside(ax, bx, &r.1.inside) {
                            valid = false;
                            break 'outer;
                        }
                    }
                    if valid {
                        let dx = bx - ax + 1;
                        result = result.max(dx * dy);
                    }
                }
            }
        }
    }

    result!(result);
    // 24555072 too low
}
