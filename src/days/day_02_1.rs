#[allow(unused_imports)]
use crate::utils::*;

pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../../input/day_02.txt");
    // let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    let mut result = 0;
    for (start, end) in input.split(',').map(|r| r.split_once('-').unwrap()) {
        let end = parse_u(end);
        let first_prefix = if start.len() % 2 == 0 {
            let (a, b) = start.split_at(start.len() / 2);
            let a = parse_u(a);
            let b = parse_u(b);
            if a >= b {
                a
            } else {
                // "<a><a>" is not in the range => pre-increment by 1
                a + 1
            }
        } else {
            // if the number is three digits ABCDE, the next value is 100100
            10usize.pow(start.len() as u32 / 2)
        };

        result += (first_prefix..)
            .map(|prefix| {
                let digits = prefix.ilog10() + 1;
                prefix * 10usize.pow(digits) + prefix
            })
            .take_while(|value| *value <= end)
            .sum::<usize>();
    }

    result!(result);
}
