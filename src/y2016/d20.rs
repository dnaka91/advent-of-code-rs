//! # Day 20: Firewall Rules
//!
//! You'd like to set up a small hidden computer here so you can use it to get back into the network
//! later. However, the corporate firewall only allows communication with certain external
//! [IP addresses].
//!
//! You've retrieved the list of blocked IPs from the firewall, but the list seems to be messy and
//! poorly maintained, and it's not clear which IPs are allowed. Also, rather than being written in
//! [dot-decimal] notation, they are written as plain [32-bit integers], which can have any value
//! from `0` through `4294967295`, inclusive.
//!
//! For example, suppose only the values `0` through `9` were valid, and that you retrieved the
//! following blacklist:
//!
//! ```txt
//! 5-8
//! 0-2
//! 4-7
//! ```
//!
//! The blacklist specifies ranges of IPs (inclusive of both the start and end value) that are
//! **not** allowed. Then, the only IPs that this firewall allows are `3` and `9`, since those are
//! the only numbers not in any range.
//!
//! Given the list of blocked IPs you retrieved from the firewall (your puzzle input), **what is the
//! lowest-valued IP** that is not blocked?
//!
//! [IP addresses]: https://en.wikipedia.org/wiki/IPv4#Addressing
//! [dot-decimal]: https://en.wikipedia.org/wiki/Dot-decimal_notation
//! [32-bit integers]: https://en.wikipedia.org/wiki/32-bit

use anyhow::Result;

pub const INPUT: &str = include_str!("d20.txt");

pub fn solve_part_one(input: &str) -> Result<i64> {
    Ok(0)
}

pub fn solve_part_two(input: &str) -> Result<i64> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {}

    #[test]
    fn part_two() {}
}
