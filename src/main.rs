
mod day;
mod days;

use std::rc::Rc;

use seq_macro::seq;
use compose_idents::compose;

macro_rules! run_day {
    ($day:ident) => {{
        const DAY: &str = stringify!($day);
        const IN: &str = include_str!(concat!("../inputs/", stringify!($day), ".txt"));
        println!("=== {} ===", DAY);
        let p1out = days::$day::do_part1(IN);
        match p1out {
            Ok(v) => println!("part 1: {}", v),
            Err(_) => println!("part 1: ERROR")
        }
        let p2out = days::$day::do_part2(IN);
        match p2out {
            Ok(v) => println!("part 2: {}", v),
            Err(_) => println!("part 2: ERROR")
        }
    }}
}

macro_rules! run_days {
    ($day:expr, $day_num:expr) => {
        seq!(N in 1..=$day {
            if $day_num == 0 || $day_num == N {
                compose!(
                    day = concat(day, N),
                    {
                        run_day!(day)
                    }
                );
            }
        });
        if $day_num != 0 && $day_num > $day {
            println!("Day {} is not implemented.", $day_num);
        }
    };
}

fn main() {
    let args: Rc<[String]> = std::env::args().collect();
    let day_num: u32;
    if args.len() == 2 {
        day_num = u32::from_str_radix(&args[1], 10).unwrap();
    } else {
        day_num = 0;
    }
    run_days!(7, day_num);
}
