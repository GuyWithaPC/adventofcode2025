use std::sync::Arc;

use crate::day;

day!{
{
    part1 <-
"987654321111111
811111111111119
234234234234278
818181911112111"
=> 357; u64
},
{
    part2 <-
"987654321111111
811111111111119
234234234234278
818181911112111"
=> 3121910778619; u64
}
}

fn convert_bank(bank: &str) -> Arc<[u64]> {
    bank.chars().map(|c| c.to_digit(10).unwrap() as u64).collect()
}

fn part1(input: &str) -> u64 {
    input.lines().map(convert_bank).map(|b| largest_dyn(b.as_ref(), 2)).sum()
}

fn max(slice: &[u64]) -> u64 {
    *slice.iter().max().unwrap()
}

fn largest_dyn(bank: &[u64], size: usize) -> u64 {
    let l = bank.len();
    let mut matrix = vec![0; l * size];
    for i in 0..size {
        let place = 10u64.pow(i as u32);
        for j in 0..(l-i) {
            if i == 0 {
                matrix[i * l + j] = bank[j]
            } else {
                matrix[i * l + j] = place * bank[j] + max(&matrix[(i - 1) * l + j + 1..i*l])
            }
        }
    }
    max(&matrix[(size - 1) * l..])
}

fn part2(input: &str) -> u64 {
    input.lines().map(convert_bank).map(|b| largest_dyn(b.as_ref(), 12)).sum()
}