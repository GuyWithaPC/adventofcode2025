
use std::fmt::Display;

use crate::day;
use itertools::Itertools;

day!{
{
    part1 <-
"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."
=> 13; usize
},
{
    part2 <-
"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."
=> 43; usize
}
}

struct Grid {
    data: Box<[bool]>,
    width: usize,
    height: usize
}

fn neighbors(x: isize, y: isize) -> impl Iterator<Item = (isize, isize)> {
    (x-1..=x+1).cartesian_product(y-1..=y+1)
        .filter(move |(xi, yi)| !(*xi == x && *yi == y))
}

impl Grid {
    pub fn init(grid: &str) -> Self {
        let height = grid.lines().count();
        let width = grid.lines().next().unwrap().len();
        let data = grid.chars()
            .filter_map(|c| match c {
                '.' => Some(false),
                '@' => Some(true),
                _ => None
            }).collect();
        Self {
            data,
            width,
            height
        }
    }

    pub fn locations(&self) -> impl Iterator<Item = (isize, isize)> {
        (0..self.height as isize).cartesian_product(0..self.width as isize)
    }

    pub fn get(&self, x: isize, y: isize) -> bool {
        if y >= self.height as isize || x >= self.width as isize || y < 0 || x < 0 {
            return false;
        }
        let idx = y as usize * self.width + x as usize;
        self.data[idx]
    }

    pub fn iterate(&mut self) -> usize {
        let mut removed = 0;
        let new_data = self.locations().map(|(y, x)| {
            self.get(x, y) && if self.count_neighbors(x, y) < 4 {
                removed += 1;
                false
            } else {
                true
            }
        }).collect();
        self.data = new_data;
        removed
    }

    pub fn count_neighbors(&self, x: isize, y: isize) -> usize {
        neighbors(x, y).filter(|(xn, yn)| {
            self.get(*xn, *yn)
        }).count()
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", if self.get(x as isize, y as isize) {'@'} else {'.'})?
            }
            if y != self.height - 1 {
                writeln!(f)?
            }
        }
        todo!()
    }
}

fn part1(input: &str) -> usize {
    let mut grid = Grid::init(input);
    grid.iterate()
}

fn part2(input: &str) -> usize {
    let mut grid = Grid::init(input);
    let mut removed = 0;
    loop {
        let r = grid.iterate();
        if r == 0 {
            break;
        }
        removed += r;
    }
    removed
}