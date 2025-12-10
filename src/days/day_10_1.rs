#[allow(unused_imports)]
use crate::utils::*;

type Mask = u16;

fn parse_indicators(indicators: &str) -> u16 {
    indicators
        .chars()
        .enumerate()
        .filter(|(_, c)| *c == '#')
        .map(|(i, _)| 1 << i)
        .sum()
}

fn parse_button(button: &str) -> Mask {
    let inner = button.strip_prefix('(').unwrap().strip_suffix(')').unwrap();
    let numbers = inner.split(',').map(parse_u);
    numbers.fold(0, |acc, n| acc | (1 << n))
}

pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../../input/day_10.txt");
    //     let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
    // [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
    // [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    // let mut grid = _grid(input);

    let machines = input
        .lines()
        .map(|l| sscanf!(l, "[{str}] {str} {{{str}}}").unwrap())
        .map(|(indicators, buttons, joltage)| {
            let num_indicators = indicators.len();

            let indicators = parse_indicators(indicators);
            let buttons = buttons.split_ascii_whitespace().map(parse_button).to_vec();
            let joltage = joltage.split(',').map(parse_u).to_vec();

            assert_eq!(num_indicators, joltage.len());

            (indicators, buttons, joltage)
        })
        .to_vec();

    let mut result = 0;
    for (target, buttons, _joltage) in &machines {
        // buttons toggle, so pressing the same button twice is useless.
        // => Figure out the combination of buttons to press once through the bits of a number
        let num_combinations = 1u32 << buttons.len();
        let Some(best) = (0..num_combinations)
            .filter(|x| {
                let result = buttons
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| (x & (1 << i)) != 0)
                    .fold(0, |acc, (_, mask)| acc ^ mask);
                result == *target
            })
            .map(|x| x.count_ones())
            .min()
        else {
            let buttons = buttons.iter().map(|b| format!("{:010b}", b)).to_vec();
            panic!(
                "Unable to get {} {:010b} from {:?} in {} combinations",
                target, target, buttons, num_combinations
            );
        };
        result += best;
    }

    result!(result);
}
