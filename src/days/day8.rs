
use std::sync::{Arc, Mutex};

use itertools::Itertools;

use crate::day;

day!{
{
    part1 <-
"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"
=> 40; u64
},
{
    part2 <-
"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"
=> 25272; u64
}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Group {
    Reference(usize),
    Leader
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Node {
    position: (i32, i32, i32),
    group: Group
}
impl Node {
    pub fn from_line(line: &str) -> Self {
        let (x, line) = line.split_once(',').unwrap();
        let (y, z) = line.split_once(',').unwrap();
        let x = i32::from_str_radix(x, 10).unwrap();
        let y = i32::from_str_radix(y, 10).unwrap();
        let z = i32::from_str_radix(z, 10).unwrap();
        Self {
            position: (x, y, z),
            group: Group::Leader
        }
    }
    pub fn distance_to(&self, other: &Node) -> f32 {
        let x = (self.position.0 - other.position.0) as f32;
        let y = (self.position.1 - other.position.1) as f32;
        let z = (self.position.2 - other.position.2) as f32;
        f32::sqrt(x * x + y * y + z * z)
    }
}
impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}[{:?}]", self.position, self.group)
    }
}

#[derive(Clone)]
struct Graph {
    nodes: Vec<Node>
}
impl Graph {
    pub fn from_input(input: &str) -> Self {
        Self {
            nodes: input.lines().map(Node::from_line).collect()
        }
    }
    pub fn join_closest(&mut self, count: usize) {
        let joined: Mutex<usize> = Mutex::new(0);
        self.closest_nodes().take_while(|_| *joined.lock().unwrap() < count)
            .for_each(|(a, b)| {
                self.join(a, b);
                *joined.lock().unwrap() += 1;
            })
    }
    pub fn join_all(&mut self) -> u64 {
        let (a, b) = self.closest_nodes().fold(None::<(Node, Node)>, |last, (a, b)| {
            if self.get_leader(a) != self.get_leader(b) {
                self.join(a, b);
                Some((self.nodes[a], self.nodes[b]))
            } else {
                last
            }
        }).unwrap();
        a.position.0 as u64 * b.position.0 as u64
    }
    pub fn largest_groups(&self) -> Box<dyn Iterator<Item = u64>> {
        Box::new((0..self.nodes.len()).sorted_by(|a, b| {
            self.get_leader(*a).cmp(&self.get_leader(*b))
        }).dedup_by_with_count(|a, b| {
            self.get_leader(*a) == self.get_leader(*b)
        }).sorted_by(|(a, _), (b, _)| {
            b.cmp(a)
        }).map(|(count, _)| count as u64))
    }
    pub fn closest_nodes(&self) -> Box<dyn Iterator<Item = (usize, usize)>> {
        Box::new((0..self.nodes.len()).tuple_combinations().map(|(a, b)| {
            let dist = self.nodes[a].distance_to(&self.nodes[b]);
            ((a, b), dist)
        }).sorted_by(|(_, a), (_, b)| {
            a.total_cmp(b)
        }).map(|(p, _)| p))
    }
    pub fn get_leader(&self, node: usize) -> usize {
        match self.nodes[node].group {
            Group::Leader => node,
            Group::Reference(n) => self.get_leader(n)
        }
    }
    pub fn join(&mut self, a: usize, b: usize) {
        let a_leader = self.get_leader(a);
        let b_leader = self.get_leader(b);
        if a_leader != b_leader {
            self.nodes[a_leader].group = Group::Reference(b_leader);
        }
    }
}

#[cfg(test)]
const NUM: usize = 10;
#[cfg(not(test))]
const NUM: usize = 1000;

fn part1(input: &str) -> u64 {
    let mut graph = Graph::from_input(input);
    graph.join_closest(NUM);
    graph.largest_groups().take(3).fold(1, |prod, n| prod * n)
}

fn part2(input: &str) -> u64 {
    let mut graph = Graph::from_input(input);
    graph.join_all()
}
