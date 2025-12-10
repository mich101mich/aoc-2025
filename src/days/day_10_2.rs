#[allow(unused_imports)]
use crate::utils::*;

type State = [u16; 10];

struct Button {
    indices: Vec<usize>,
    set: u16,
}
impl Button {
    fn new(s: &str) -> Self {
        let inner = s.strip_prefix('(').unwrap().strip_suffix(')').unwrap();
        let indices = inner.split(',').map(parse_u).to_vec();
        let set = indices.iter().fold(0, |acc, x| acc | (1 << x));
        Self { indices, set }
    }
    fn has(&self, x: usize) -> bool {
        (self.set & (1 << x)) != 0
    }
    fn press_limit(&self, target: &State) -> u16 {
        self.indices.iter().map(|i| target[*i]).min().unwrap()
    }
}
impl std::fmt::Debug for Button {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Button")
            .field("indices", &self.indices)
            .field("set", &format!("{:010b}", self.set))
            .finish()
    }
}

pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../../input/day_10.txt");
    //     let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
    // [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
    // [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    let machines = input
        .lines()
        .map(|l| sscanf!(l, "[{str}] {str} {{{str}}}").unwrap())
        .map(|(_indicators, buttons, joltage)| {
            let buttons = buttons.split_ascii_whitespace().map(Button::new).to_vec();
            let joltage = joltage.split(',').map(parse_u).map(|x| x as u16).to_vec();

            (buttons, joltage)
        })
        .to_vec();

    fn solve(target: State, cost: usize, buttons: &mut [Button], best: &mut usize) {
        let biggest = *target.iter().max().unwrap();
        if cost + biggest as usize >= *best {
            return;
        } else if biggest == 0 {
            *best = cost + biggest as usize;
            return;
        }

        let given_buttons = buttons.iter().fold(0, |acc, b| acc | b.set);
        if target
            .iter()
            .enumerate()
            .any(|(i, n)| *n > 0 && (given_buttons & (1 << i)) == 0)
        {
            return;
        }

        let Some((current, rest)) = buttons.split_first_mut() else {
            return;
        };

        let mut remaining_limit = State::default();
        for b in rest.iter() {
            let limit = b.press_limit(&target);
            for i in &b.indices {
                remaining_limit[*i] += limit;
            }
        }
        let min_presses = current
            .indices
            .iter()
            .filter_map(|i| target[*i].checked_sub(remaining_limit[*i]))
            .max()
            .unwrap_or(0);

        let max_presses = current.press_limit(&target);

        if min_presses > max_presses {
            return;
        }

        for count in (min_presses..=max_presses).rev() {
            let mut next = target;
            for i in &current.indices {
                next[*i] -= count;
            }
            solve(next, cost + count as usize, rest, best);
        }
    }

    let result: usize = machines
        .into_par_iter()
        .map(|(mut buttons, joltage)| {
            let mut target = State::default();
            target[0..joltage.len()].copy_from_slice(&joltage);

            let smallest_index = target
                .iter()
                .enumerate()
                .filter(|(_, x)| **x > 0)
                .min_by_key(|(_, x)| **x)
                .unwrap()
                .0;
            buttons.sort_unstable_by(|a, b| {
                a.has(smallest_index)
                    .cmp(&b.has(smallest_index))
                    .then(a.indices.len().cmp(&b.indices.len()))
                    .reverse()
            });

            let mut best = usize::MAX;
            solve(target, 0, &mut buttons, &mut best);
            assert_ne!(best, usize::MAX);
            best
        })
        .sum();

    result!(result);
}
