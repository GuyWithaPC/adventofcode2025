
use crate::day;

day!{
{
    part1 <-
".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."
=> 21; u64
},
{
    part2 <-
".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."
=> 40; u64
}
}

#[derive(Debug, Clone, Copy)]
enum TachyonSpot {
    Empty,
    Tachyon(u64),
    ActiveTachyon(u64),
    Splitter
}

#[derive(Clone)]
struct TachyonGrid {
    width: usize,
    height: usize,
    grid: Vec<TachyonSpot>
}

impl TachyonGrid {
    pub fn from_input(input: &str) -> Self {
        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();
        let grid = input.chars().filter(|c| !c.is_whitespace()).map(|c| match c {
            'S' => TachyonSpot::ActiveTachyon(1),
            '^' => TachyonSpot::Splitter,
            _ => TachyonSpot::Empty
        }).collect();
        Self {
            width,
            height,
            grid
        }
    }

    pub fn get(&self, x: isize, y: isize) -> Option<TachyonSpot> {
        if x >= self.width as isize || y >= self.height as isize || x < 0 || y < 0 {
            None
        } else {
            Some(self.grid[y as usize * self.width + x as usize])
        }
    }

    fn set(&mut self, x: isize, y: isize, value: TachyonSpot) {
        let Ok(x): Result<usize, _> = x.try_into() else {
            return
        };
        let Ok(y): Result<usize, _> = y.try_into() else {
            return
        };
        self.grid[y * self.width + x] = value;
    }

    fn add_active(&mut self, x: isize, y: isize, n: u64) {
        let Ok(x): Result<usize, _> = x.try_into() else {
            return
        };
        let Ok(y): Result<usize, _> = y.try_into() else {
            return
        };
        self.grid[y * self.width + x] = match self.grid[y * self.width + x] {
            TachyonSpot::Tachyon(m)
            | TachyonSpot::ActiveTachyon(m) => TachyonSpot::ActiveTachyon(m+n),
            _ => TachyonSpot::ActiveTachyon(n)
        }
    }

    pub fn iterate_p1(&mut self) -> Option<u64> {
        let mut new_grid = self.clone();
        let mut end = false;
        let mut splits = 0u64;
        self.grid.iter().enumerate().map(|(i, g)| {
            (((i % self.width) as isize, (i / self.width) as isize), g)
        }).for_each(|((x, y), spot)| {
            match *spot {
                TachyonSpot::ActiveTachyon(n) => {
                    new_grid.set(x, y, TachyonSpot::Tachyon(n));
                    match self.get(x, y+1) {
                        Some(TachyonSpot::Splitter) => {
                            new_grid.set(x-1, y+1, TachyonSpot::ActiveTachyon(n));
                            new_grid.set(x+1, y+1, TachyonSpot::ActiveTachyon(n));
                            splits += 1;
                        }
                        Some(TachyonSpot::Empty) => {
                            new_grid.set(x, y+1, TachyonSpot::ActiveTachyon(n));
                        },
                        Some(_) => {},
                        None => end = true
                    }
                },
                _ => {}
            }
        });
        *self = new_grid;
        if end {
            None
        } else {
            Some(splits)
        }
    }

    pub fn iterate_p2(&mut self) -> bool {
        let mut new_grid = self.clone();
        let mut end = false;
        self.grid.iter().enumerate().map(|(i, g)| {
            (((i % self.width) as isize, (i / self.width) as isize), g)
        }).for_each(|((x, y), spot)| {
            match *spot {
                TachyonSpot::ActiveTachyon(n) => {
                    new_grid.set(x, y, TachyonSpot::Tachyon(n));
                    match self.get(x, y+1) {
                        Some(TachyonSpot::Splitter) => {
                            new_grid.add_active(x-1, y+1, n);
                            new_grid.add_active(x+1, y+1, n);
                        }
                        Some(TachyonSpot::Empty) => {
                            new_grid.add_active(x, y+1, n);
                        },
                        Some(_) => {},
                        None => end = true
                    }
                },
                _ => {}
            }
        });
        *self = new_grid;
        end
    }

    pub fn count_timelines(&self) -> u64 {
        let mut sum = 0u64;
        for x in 0..self.width {
            sum += match self.get(x as isize, self.height as isize - 1).unwrap() {
                TachyonSpot::Tachyon(n) => n,
                _ => 0
            };
        }
        sum
    }
}

impl std::fmt::Debug for TachyonGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                match self.get(x as isize, y as isize).unwrap() {
                    TachyonSpot::ActiveTachyon(_) => write!(f, "#")?,
                    TachyonSpot::Tachyon(n) => write!(f, "{n}")?,
                    TachyonSpot::Splitter => write!(f, "^")?,
                    TachyonSpot::Empty => write!(f, ".")?
                }
            }
            if y != self.height - 1 {
                writeln!(f)?
            }
        }
        Ok(())
    }
}

fn part1(input: &str) -> u64 {
    let mut grid = TachyonGrid::from_input(input);
    let mut sum = 0u64;
    while let Some(splits) = grid.iterate_p1() {
        sum += splits;
    }
    sum
}

fn part2(input: &str) -> u64 {
    let mut grid = TachyonGrid::from_input(input);
    while !grid.iterate_p2() {};
    grid.count_timelines()
}