#[allow(unused_imports)]
use crate::utils::*;

pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../../input/day_02.txt");
    // let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    let input = input
        .split(',')
        .map(|r| r.split_once('-').unwrap())
        .to_vec();

    let mut result = 0;
    for (start_str, end_str) in &input {
        let start = parse_u(start_str);
        let end = parse_u(end_str);
        let mut seen = HashSet::new();
        for reps in 2..=end_str.len() {
            result += (1usize..)
                .map(|prefix| {
                    let digits = prefix.ilog10() + 1;
                    let increment = 10usize.pow(digits);
                    let mut full = 0;
                    let mut factor = 1;
                    for _ in 0..reps {
                        full += prefix * factor;
                        factor *= increment;
                    }
                    full
                })
                .skip_while(|value| *value < start)
                .take_while(|value| *value <= end)
                .filter(|value| seen.insert(value.to_string()))
                .sum::<usize>();
        }
    }

    result!(result);
}
