
use std::ops::RangeInclusive;

use crate::day;

day!{
{
    part1 <-
"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
=> 1227775554; u64
},
{
    part2 <-
"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
=> 4174379265; u64
}
}

fn is_invalid(id: &str) -> bool {
    if id.is_empty() || id.len() % 2 != 0 {
        return false;
    }
    let l = id.len();
    &id[0..l/2] == &id[l/2..]
}

fn id_range(range: &str) -> RangeInclusive<u64> {
    let (l, r) = range.split_once('-').unwrap();
    u64::from_str_radix(l, 10).unwrap()..=u64::from_str_radix(r, 10).unwrap()
}

fn part1(input: &str) -> u64 {
    input.split(',').map(id_range).flatten().filter(|v| is_invalid(&v.to_string())).sum()
}

fn is_invalid_p2(id: &str) -> bool {
    if id.is_empty() {
        return false;
    }
    'tries: for l in 1..=id.len()/2 {
        if id.len() % l != 0 {
            continue;
        }
        let expect_reps = id.len() / l;
        for p in 1..expect_reps {
            if &id[0..l] != &id[p*l..p*l+l] {
                continue 'tries
            }
        }
        return true;
    }
    return false;
}

fn part2(input: &str) -> u64 {
    input.split(',').map(id_range).flatten().filter(|v| is_invalid_p2(&v.to_string())).sum()
}