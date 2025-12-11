
use itertools::Itertools;

use crate::day;

day!{
{
    part1 <-
"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"
=> 50; u64
},
{
    part2 <-
"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"
=> 24; u64
}
}

fn tiles(input: &str) -> Vec<(u64, u64)> {
    input.lines().map(|l| {
        let (x, y) = l.split_once(',').unwrap();
        (
            u64::from_str_radix(x, 10).unwrap(),
            u64::from_str_radix(y, 10).unwrap()
        )
    }).collect()
}

fn area(a: (u64, u64), b: (u64, u64)) -> u64 {
    (a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1)
}

fn part1(input: &str) -> u64 {
    let tiles = tiles(input);
    tiles.iter().tuple_combinations().map(|(a, b)| {
        area(*a, *b)
    }).k_largest(1).next().unwrap()
}

fn part2(input: &str) -> u64 {
    0
}