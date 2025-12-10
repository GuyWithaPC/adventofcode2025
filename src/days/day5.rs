use std::ops::RangeInclusive;

use crate::day;

day!{
{
    part1 <-
"3-5
10-14
16-20
12-18

1
5
8
11
17
32" =>
    3; u64
}, {
    part2 <-
"3-5
10-14
16-20
12-18

1
5
8
11
17
32" =>
    14; u64
}
}

type FreshRanges = Vec<RangeInclusive<u64>>;

fn is_fresh(id: u64, ranges: &FreshRanges) -> bool {
    ranges.iter().any(|r| r.contains(&id))
}

fn read_ranges(input: &str) -> FreshRanges {
    input.lines().take_while(|l| !l.is_empty())
        .map(|line| {
            let (l, r) = line.split_once('-').unwrap();
            u64::from_str_radix(l, 10).unwrap()..=u64::from_str_radix(r, 10).unwrap()
        }).collect()
}

fn read_ids(input: &str) -> Vec<u64> {
    input.lines().skip_while(|l| !l.is_empty()).skip(1)
        .map(|line| {
            u64::from_str_radix(line, 10).unwrap()
        }).collect()
}

fn part1(input: &str) -> u64 {
    let ranges = read_ranges(input);
    let ids = read_ids(input);
    ids.iter().map(|id| is_fresh(*id, &ranges) as u64).sum()
}

fn sort_ranges(mut ranges: FreshRanges) -> FreshRanges {
    ranges.sort_by(|a, b| {
        a.start().cmp(b.start())
    });
    ranges
}

fn merge_overlap(ranges: FreshRanges) -> FreshRanges {
    let mut new_ranges = Vec::new();
    let mut i = 0usize;
    while i < ranges.len() {
        let start = *ranges[i].start();
        let mut end = *ranges[i].end();
        while i < ranges.len() - 1 && end >= *ranges[i+1].start() {
            end = u64::max(*ranges[i+1].end(), end);
            i += 1;
        }
        new_ranges.push(start..=end);
        i += 1;
    }
    new_ranges
}

fn part2(input: &str) -> u64 {
    let ranges = read_ranges(input);
    let merged = merge_overlap(sort_ranges(ranges));
    let mut fresh: u64 = 0u64;
    for range in merged {
        fresh += (range.end() - range.start()) + 1;
    }
    fresh
}