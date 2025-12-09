use std::collections::{BinaryHeap, HashSet};

pub fn parse(input: &str) -> Vec<Vec3> {
    input
        .lines()
        .map(|l| {
            let mut coords = l.split(",").map(|str| str.parse().unwrap());

            let x = coords.next().unwrap();
            let y = coords.next().unwrap();
            let z = coords.next().unwrap();
            Vec3(x, y, z)
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec3(i64, i64, i64);

impl Vec3 {
    /// Calculates the squared Euclidean distance: (dx^2 + dy^2 + dz^2)
    #[inline]
    pub fn distance_to(&self, rhs: Vec3) -> i64 {
        let d_x = self.0 - rhs.0;
        let d_x = d_x.pow(2);

        let d_y = self.1 - rhs.1;
        let d_y = d_y.pow(2);

        let d_z = self.2 - rhs.2;
        let d_z = d_z.pow(2);

        d_x + d_y + d_z
    }
}

pub fn part_1(coords: &[Vec3]) -> u64 {
    let distances: Vec<_> = coords
        .iter()
        .map(|&p| {
            // let heap_distance: BinaryHeap<(i64, Vec3)> = coords
            //     .iter()
            //     .filter(|b| **b != p)
            //     .map(|&b| (p.distance_to(b), b))
            //     .collect();

            // let mut sorted_distances: BinaryHeap<i64> = distances_iterator.collect();

            // let mut distances_vec: Vec<i64> = sorted_distances.drain().collect();
            // distances_vec.reverse();

            // distances_vec
        })
        .collect();

    dbg!(distances);

    todo!();
}

pub fn part_2(coords: &[Vec3]) -> u64 {
    todo!()
}
