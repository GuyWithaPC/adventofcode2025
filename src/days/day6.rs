
use crate::day;

day!{
{
    part1 <-
"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  " =>
    4277556; u64
}, {
    part2 <-
"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  " =>
    3263827; u64
}
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Mul,
    Add
}

fn parse_input_p1(input: &str) -> (Vec<Vec<u64>>, Vec<Operator>) {
    let lines: Vec<&str> = input.lines().collect();
    let len = lines.len();
    let ops: Vec<Operator> = lines[len-1].split_whitespace().filter(|o| !o.is_empty()).map(|o| match o {
        "*" => Operator::Mul,
        "+" => Operator::Add,
        _ => panic!("unsupported operator character")
    }).collect();
    let width = ops.len();
    let mut nums = vec![Vec::new();width];
    for i in 0..len-1 {
        lines[i].split_whitespace()
            .filter(|n| !n.is_empty()).enumerate()
            .for_each(|(i, n)| {
                let num = u64::from_str_radix(n, 10).unwrap();
                nums[i].push(num);
            });
    }
    (nums, ops)
}

fn part1(input: &str) -> u64 {
    let (nums, ops) = parse_input_p1(input);
    ops.iter().enumerate().map(|(col, op)| {
        match *op {
            Operator::Add => nums[col].iter().sum(),
            Operator::Mul => nums[col].iter().fold(1, |prod, n| prod * n)
        }
    }).sum()
}

fn columns(input: &str) -> Vec<String> {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let chars: Vec<char> = input.lines().map(|l| l.chars()).flatten().collect();
    let mut cols = Vec::new();
    for x in 0..width {
        let mut col = Vec::new();
        for y in 0..height {
            col.push(chars[y * width + x])
        }
        cols.push(col.iter().collect())
    }
    cols
}

fn parse_columns(input: &str) -> Vec<(Operator, Vec<u64>)> {
    let cols = columns(input);
    let mut out = Vec::new();
    let mut op = None::<Operator>;
    let mut nums = Vec::new();
    for col in cols {
        if col.chars().all(|c| c.is_whitespace()) {
            out.push((op.unwrap(), nums));
            nums = Vec::new();
            op = None;
            continue;
        }
        let num: String = col.chars()
            .skip_while(|c| c.is_whitespace()).take_while(|c| c.is_numeric())
            .collect();
        let num: u64 = u64::from_str_radix(&num, 10).unwrap();
        nums.push(num);
        match col.chars().last().unwrap() {
            '*' => op = Some(Operator::Mul),
            '+' => op = Some(Operator::Add),
            _ => {}
        }
    }
    out.push((op.unwrap(), nums));
    out
}

fn part2(input: &str) -> u64 {
    let equations = parse_columns(input);
    equations.iter().map(|(op, nums)| match op {
        Operator::Add => nums.iter().sum(),
        Operator::Mul => nums.iter().fold(1, |prod, n| prod * n)
    }).sum()
}