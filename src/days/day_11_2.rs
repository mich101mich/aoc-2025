#[allow(unused_imports)]
use crate::utils::*;

pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../../input/day_11.txt");
    // let input = "";

    // let mut grid = _grid(input);

    let devices = input
        .lines()
        .map(|l| sscanf!(l, "{str}: {str}").unwrap())
        .map(|(k, v)| (k, v.split_ascii_whitespace().to_vec()))
        .to_map();

    #[derive(Default)]
    struct Counts {
        dac: usize,
        fft: usize,
        out: usize,
    }

    let mut pos = HashMap::new();
    let mut next_pos = HashMap::new();
    fn search(
        pos: &mut HashMap<&'static str, usize>,
        next_pos: &mut HashMap<&'static str, usize>,
        devices: &HashMap<&'static str, Vec<&'static str>>,
    ) -> Counts {
        let mut counts = Counts::default();
        while !pos.is_empty() {
            for (p, n) in pos.iter() {
                for &next in devices
                    .get(p)
                    .unwrap_or_else(|| panic!("unknown: {p}"))
                    .iter()
                {
                    match next {
                        "dac" => counts.dac += n,
                        "fft" => counts.fft += n,
                        "out" => counts.out += n,
                        _ => *next_pos.entry(next).or_default() += n,
                    }
                }
            }
            std::mem::swap(pos, next_pos);
            next_pos.clear();
        }
        counts
    }

    pos.insert("svr", 1);
    let Counts { dac, fft, .. } = search(&mut pos, &mut next_pos, &devices);

    pos.insert("fft", fft);
    let fft_dac = search(&mut pos, &mut next_pos, &devices).dac;

    pos.insert("dac", dac);
    let dac_fft = search(&mut pos, &mut next_pos, &devices).fft;

    let mut result = 0;
    pos.insert("fft", dac_fft);
    result += search(&mut pos, &mut next_pos, &devices).out;
    pos.insert("dac", fft_dac);
    result += search(&mut pos, &mut next_pos, &devices).out;

    result!(result);
}
