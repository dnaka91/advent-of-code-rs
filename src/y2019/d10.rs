//! # Day 10: Monitoring Station
//!
//! You fly into the asteroid belt and reach the Ceres monitoring station. The Elves here have an
//! emergency: they're having trouble tracking all of the asteroids and can't be sure they're safe.
//!
//! The Elves would like to build a new monitoring station in a nearby area of space; they hand you
//! a map of all of the asteroids in that region (your puzzle input).
//!
//! The map indicates whether each position is empty (`.`) or contains an asteroid (`#`). The
//! asteroids are much smaller than they appear on the map, and every asteroid is exactly in the
//! center of its marked position. The asteroids can be described with `X,Y` coordinates where `X`
//! is the distance from the left edge and `Y` is the distance from the top edge (so the top-left
//! corner is `0,0` and the position immediately to its right is `1,0`).
//!
//! Your job is to figure out which asteroid would be the best place to build a **new monitoring
//! station**. A monitoring station can **detect** any asteroid to which it has **direct line of
//! sight** - that is, there cannot be another asteroid **exactly** between them. This line of sight
//! can be at any angle, not just lines aligned to the grid or diagonally. The **best** location is
//! the asteroid that can **detect** the largest number of other asteroids.
//!
//! For example, consider the following map:
//!
//! ```txt
//! .#..#
//! .....
//! #####
//! ....#
//! ...##
//! ```
//!
//! The best location for a new monitoring station on this map is the highlighted asteroid at `3,4`
//! because it can detect `8` asteroids, more than any other location. (The only asteroid it cannot
//! detect is the one at `1,0`; its view of this asteroid is blocked by the asteroid at` 2,2`.) All
//! other asteroids are worse locations; they can detect `7` or fewer other asteroids. Here is the
//! number of other asteroids a monitoring station on each asteroid could detect:
//!
//! ```txt
//! .7..7
//! .....
//! 67775
//! ....7
//! ...87
//! ```
//!
//! Here is an asteroid (`#`) and some examples of the ways its line of sight might be blocked. If
//! there were another asteroid at the location of a capital letter, the locations marked with the
//! corresponding lowercase letter would be blocked and could not be detected:
//!
//! ```txt
//! #.........
//! ...A......
//! ...B..a...
//! .EDCG....a
//! ..F.c.b...
//! .....c....
//! ..efd.c.gb
//! .......c..
//! ....f...c.
//! ...e..d..c
//! ```
//!
//! Here are some larger examples:
//!
//! - Best is `5,8` with `33` other asteroids detected:
//!
//! ```txt
//! ......#.#.
//! #..#.#....
//! ..#######.
//! .#.#.###..
//! .#..#.....
//! ..#....#.#
//! #..#....#.
//! .##.#..###
//! ##...#..#.
//! .#....####
//! ```
//!
//! - Best is `1,2` with `35` other asteroids detected:
//!
//! ```txt
//! #.#...#.#.
//! .###....#.
//! .#....#...
//! ##.#.#.#.#
//! ....#.#.#.
//! .##..###.#
//! ..#...##..
//! ..##....##
//! ......#...
//! .####.###.
//! ```
//!
//! - Best is `6,3` with `41` other asteroids detected:
//!
//! ```txt
//! .#..#..###
//! ####.###.#
//! ....###.#.
//! ..###.##.#
//! ##.##.#.#.
//! ....###..#
//! ..#.#..#.#
//! #..#.#.###
//! .##...##.#
//! .....#.#..
//! ```
//!
//! - Best is `11,13` with `210` other asteroids detected:
//!
//! ```txt
//! .#..##.###...#######
//! ##.############..##.
//! .#.######.########.#
//! .###.#######.####.#.
//! #####.##.#.##.###.##
//! ..#####..#.#########
//! ####################
//! #.####....###.#.#.##
//! ##.#################
//! #####.##.###..####..
//! ..######..##.#######
//! ####.##.####...##..#
//! .#####..#.######.###
//! ##...#.##########...
//! #.##########.#######
//! .####.#.###.###.#.##
//! ....##.##.###..#####
//! .#.#.###########.###
//! #.#.#.#####.####.###
//! ###.##.####.##.#..##
//! ```
//!
//! Find the best location for a new monitoring station. **How many other asteroids can be detected
//! from that location?**

use std::iter::FromIterator;

use anyhow::ensure;
use anyhow::Result;
use derive_more::{Add, AddAssign, Sub, SubAssign};
use fnv::{FnvHashMap, FnvHashSet};
use itertools::{cloned, Itertools};

pub const INPUT: &str = include_str!("d10.txt");

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Add, Sub, AddAssign)]
struct Point(i16, i16);

pub fn solve_part_one(input: &str) -> Result<i16> {
    let line_length = input.lines().next().unwrap().len();
    let mut asteroids = FnvHashSet::default();

    for (y, line) in input.lines().enumerate() {
        ensure!(line.len() == line_length, "all lines must have the same length");
        asteroids.extend(
            line.bytes()
                .enumerate()
                .filter(|(_, b)| *b == b'#')
                .map(|(x, _)| Point(x as i16, y as i16)),
        );
    }

    let line_count = input.lines().count();
    let mut map = FnvHashMap::from_iter(asteroids.iter().map(|a| (*a, asteroids.len() as i16 - 1)));

    for a in asteroids.iter().cloned() {
        for b in asteroids.iter().cloned().filter(|b| a != *b) {
            let mut delta = b - a;
            while delta.0 % 2 == 0 && delta.1 % 2 == 0 {
                delta.0 /= 2;
                delta.1 /= 2;
            }
            if delta.0 == 0 {
                delta.1 /= delta.1.abs();
            }
            if delta.1 == 0 {
                delta.0 /= delta.0.abs();
            }
            let mut c = b + delta;

            while c.0 >= 0
                && c.1 >= 0
                && c.0 < line_length as i16
                && c.1 < line_count as i16
                && c != a
            {
                if asteroids.contains(&c) {
                    let count = map.get_mut(&a).unwrap();
                    *count -= 1;
                    break;
                }
                c += delta;
            }
        }
    }

    println!("------");
    for y in 0..line_count as i16 {
        for x in 0..line_length as i16 {
            if let Some(c) = map.get(&Point(x, y)) {
                print!("{:02} ", c);
            } else {
                print!(".. ");
            }
        }
        println!();
    }
    println!("------");

    Ok(*map.values().max().unwrap())
}

pub fn solve_part_two(input: &str) -> Result<i64> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    fn part_one() {
        let input = ".#..#\n\
                     .....\n\
                     #####\n\
                     ....#\n\
                     ...##\n";
        assert_eq!(8, solve_part_one(input).unwrap());

        let input = "#.#...#.#.\n\
                     .###....#.\n\
                     .#....#...\n\
                     ##.#.#.#.#\n\
                     ....#.#.#.\n\
                     .##..###.#\n\
                     ..#...##..\n\
                     ..##....##\n\
                     ......#...\n\
                     .####.###.\n";
        assert_eq!(35, solve_part_one(input).unwrap());

        let input = ".#..#..###\n\
                     ####.###.#\n\
                     ....###.#.\n\
                     ..###.##.#\n\
                     ##.##.#.#.\n\
                     ....###..#\n\
                     ..#.#..#.#\n\
                     #..#.#.###\n\
                     .##...##.#\n\
                     .....#.#..\n";
        assert_eq!(41, solve_part_one(input).unwrap());

        let input = ".#..##.###...#######\n\
                     ##.############..##.\n\
                     .#.######.########.#\n\
                     .###.#######.####.#.\n\
                     #####.##.#.##.###.##\n\
                     ..#####..#.#########\n\
                     ####################\n\
                     #.####....###.#.#.##\n\
                     ##.#################\n\
                     #####.##.###..####..\n\
                     ..######..##.#######\n\
                     ####.##.####...##..#\n\
                     .#####..#.######.###\n\
                     ##...#.##########...\n\
                     #.##########.#######\n\
                     .####.#.###.###.#.##\n\
                     ....##.##.###..#####\n\
                     .#.#.###########.###\n\
                     #.#.#.#####.####.###\n\
                     ###.##.####.##.#..##\n";
        assert_eq!(210, solve_part_one(input).unwrap());
    }

    #[test]
    fn part_two() {}
}
