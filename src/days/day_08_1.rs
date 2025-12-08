#[allow(unused_imports)]
use crate::utils::*;

fn sq_euclid(a: Point3D, b: Point3D) -> usize {
    let delta = a.abs_diff(b);
    delta.dot(delta)
}

pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../../input/day_08.txt");
    //     let input = "162,817,812
    // 57,618,57
    // 906,360,560
    // 592,479,940
    // 352,342,300
    // 466,668,158
    // 542,29,236
    // 431,825,988
    // 739,650,466
    // 52,470,668
    // 216,146,977
    // 819,987,18
    // 117,168,530
    // 805,96,715
    // 346,949,466
    // 970,615,88
    // 941,993,340
    // 862,61,35
    // 984,92,344
    // 425,690,689";

    let junctions = input
        .lines()
        .map(|l| sscanf!(l, "{0},{0},{0}", usize).unwrap())
        .map(|(x, y, z)| p3(x, y, z))
        .to_vec();

    let n = junctions.len();
    let junctions = &junctions; // for the move closure

    let mut index_pairs = (0..n)
        .flat_map(|a| (a + 1..n).map(move |b| (a, b, sq_euclid(junctions[a], junctions[b]))))
        .to_vec();
    index_pairs.sort_by_key(|(_, _, d)| *d);
    // pv!(index_pairs);

    let mut parent_index = (0..n).to_vec();
    fn get_parent(x: usize, parent_index: &mut [usize]) -> usize {
        if parent_index[x] != x {
            parent_index[x] = get_parent(parent_index[x], parent_index);
        }
        parent_index[x]
    }

    for &(a, b, _) in index_pairs.iter().take(1000) {
        let pa = get_parent(a, &mut parent_index);
        let pb = get_parent(b, &mut parent_index);
        let parent = pa.min(pb);
        // pv!(a, junctions[a], b, junctions[b], pa, pb);
        parent_index[a] = parent;
        parent_index[b] = parent;
        parent_index[pa] = parent;
        parent_index[pb] = parent;
    }

    let mut circuits = HashMap::<usize, usize>::new();
    for i in 0..n {
        *circuits
            .entry(get_parent(i, &mut parent_index))
            .or_default() += 1;
    }
    let mut sizes = circuits.values().copied().to_vec();
    sizes.sort_by_key(|x| std::cmp::Reverse(*x));

    let result: usize = sizes.iter().take(3).product();
    result!(result);
}
