
use crate::day;

day!{
{
    part1 <-
"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82" =>
    3; i32
}, {
    part2 <-
"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82" =>
    6; i32
}
}

fn read_line(line: &str) -> i32 {
    if line.is_empty() {
        panic!("lines shouldn't be empty")
    }
    let (start, fin) = line.split_at(1);
    match start {
        "L" => -i32::from_str_radix(fin, 10).unwrap(),
        "R" => i32::from_str_radix(fin, 10).unwrap(),
        _ => panic!("bad line")
    }
}

fn part1(input: &str) -> i32 {
    input.lines().map(read_line).fold((50, 0), |(loc, count), instr| {
        let new_loc = (loc + instr).rem_euclid(100);
        (new_loc, count + (new_loc == 0) as i32)
    }).1
}

fn part2(input: &str) -> i32 {
    input.lines().map(read_line).fold((50, 0), |(loc, count), instr| {
        let new_loc = (loc + instr).rem_euclid(100);
        let new_count = count + (loc + instr).div_euclid(100).abs();
        (new_loc, new_count)
    }).1
}