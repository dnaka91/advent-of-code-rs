//! # Day 9: Marble Mania
//!
//! You talk to the Elves while you wait for your navigation system to initialize. To pass the time,
//! they introduce you to their favorite [marble] game.
//!
//! The Elves play this game by taking turns arranging the marbles in a **circle** according to very
//! particular rules. The marbles are numbered starting with `0` and increasing by `1` until every
//! marble has a number.
//!
//! First, the marble numbered `0` is placed in the circle. At this point, while it contains only a
//! single marble, it is still a circle: the marble is both clockwise from itself and
//! counter-clockwise from itself. This marble is designated the **current marble**.
//!
//! Then, each Elf takes a turn placing the **lowest-numbered remaining marble** into the circle
//! between the marbles that are `1` and `2` marbles **clockwise** of the current marble. (When the
//! circle is large enough, this means that there is one marble between the marble that was just
//! placed and the current marble.) The marble that was just placed then becomes the **current
//! marble**.
//!
//! However, if the marble that is about to be placed has a number which is a multiple of `23`,
//! **something entirely different happens**. First, the current player keeps the marble they would
//! have placed, adding it to their **score**. In addition, the marble `7` marbles
//! **counter-clockwise** from the current marble is **removed** from the circle and **also** added
//! to the current player's score. The marble located immediately **clockwise** of the marble that
//! was removed becomes the new **current marble**.
//!
//! For example, suppose there are 9 players. After the marble with value `0` is placed in the
//! middle, each player (shown in square brackets) takes a turn. The result of each of those turns
//! would produce circles of marbles like this, where clockwise is to the right and the resulting
//! current marble is in parentheses:
//!
//! ```txt
//! [-] (0)
//! [1]  0 (1)
//! [2]  0 (2) 1
//! [3]  0  2  1 (3)
//! [4]  0 (4) 2  1  3
//! [5]  0  4  2 (5) 1  3
//! [6]  0  4  2  5  1 (6) 3
//! [7]  0  4  2  5  1  6  3 (7)
//! [8]  0 (8) 4  2  5  1  6  3  7
//! [9]  0  8  4 (9) 2  5  1  6  3  7
//! [1]  0  8  4  9  2(10) 5  1  6  3  7
//! [2]  0  8  4  9  2 10  5(11) 1  6  3  7
//! [3]  0  8  4  9  2 10  5 11  1(12) 6  3  7
//! [4]  0  8  4  9  2 10  5 11  1 12  6(13) 3  7
//! [5]  0  8  4  9  2 10  5 11  1 12  6 13  3(14) 7
//! [6]  0  8  4  9  2 10  5 11  1 12  6 13  3 14  7(15)
//! [7]  0(16) 8  4  9  2 10  5 11  1 12  6 13  3 14  7 15
//! [8]  0 16  8(17) 4  9  2 10  5 11  1 12  6 13  3 14  7 15
//! [9]  0 16  8 17  4(18) 9  2 10  5 11  1 12  6 13  3 14  7 15
//! [1]  0 16  8 17  4 18  9(19) 2 10  5 11  1 12  6 13  3 14  7 15
//! [2]  0 16  8 17  4 18  9 19  2(20)10  5 11  1 12  6 13  3 14  7 15
//! [3]  0 16  8 17  4 18  9 19  2 20 10(21) 5 11  1 12  6 13  3 14  7 15
//! [4]  0 16  8 17  4 18  9 19  2 20 10 21  5(22)11  1 12  6 13  3 14  7 15
//! [5]  0 16  8 17  4 18(19) 2 20 10 21  5 22 11  1 12  6 13  3 14  7 15
//! [6]  0 16  8 17  4 18 19  2(24)20 10 21  5 22 11  1 12  6 13  3 14  7 15
//! [7]  0 16  8 17  4 18 19  2 24 20(25)10 21  5 22 11  1 12  6 13  3 14  7 15
//! ```
//!
//! The goal is to be the **player with the highest score** after the last marble is used up.
//! Assuming the example above ends after the marble numbered `25`, the winning score is `23+9=32`
//! (because player 5 kept marble `23` and removed marble `9`, while no other player got any points
//! in this very short example game).
//!
//! Here are a few more examples:
//!
//! - `10` players; last marble is worth `1618` points: high score is **`8317`**
//! - `13` players; last marble is worth `7999` points: high score is **`146373`**
//! - `17` players; last marble is worth `1104` points: high score is **`2764`**
//! - `21` players; last marble is worth `6111` points: high score is **`54718`**
//! - `30` players; last marble is worth `5807` points: high score is **`37305`**
//!
//! **What is the winning Elf's score?**
//!
//! [marble]: https://en.wikipedia.org/wiki/Marble_(toy)

use anyhow::Result;

pub const INPUT: &str = include_str!("d09.txt");

pub fn solve_part_one(input: &str) -> Result<u32> {
    let (player_count, last_marble) = parse_input(input)?;

    Ok(solve(player_count, last_marble))
}

pub fn solve_part_two(input: &str) -> Result<u32> {
    let (player_count, last_marble) = parse_input(input)?;

    Ok(solve(player_count, last_marble * 100))
}

fn parse_input(input: &str) -> Result<(usize, u32)> {
    let parts = input.split_whitespace().collect::<Vec<_>>();
    Ok((parts[0].parse()?, parts[6].parse()?))
}

fn solve(player_count: usize, last_marble: u32) -> u32 {
    let mut playfield = Vec::with_capacity(last_marble as usize);
    let mut players = vec![0; player_count];

    let mut current_marble = 0;
    let mut current_player = 0;
    let mut count = 0;

    playfield.push(0);

    while count < last_marble {
        count += 1;

        if count % 23 == 0 {
            current_marble = (current_marble + playfield.len() - 7) % playfield.len();
            let marble = playfield.remove(current_marble);

            players[current_player] += count + marble;
        } else {
            current_marble = (current_marble + 1) % playfield.len() + 1;

            playfield.insert(current_marble, count);
        }

        current_player = (current_player + 1) % players.len();
    }

    players.into_iter().max().unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        assert_eq!(32, solve_part_one("9 players; last marble is worth 25 points").unwrap());
        assert_eq!(8317, solve_part_one("10 players; last marble is worth 1618 points").unwrap());
        assert_eq!(
            146_373,
            solve_part_one("13 players; last marble is worth 7999 points").unwrap()
        );
        assert_eq!(2764, solve_part_one("17 players; last marble is worth 1104 points").unwrap());
        assert_eq!(54718, solve_part_one("21 players; last marble is worth 6111 points").unwrap());
        assert_eq!(37305, solve_part_one("30 players; last marble is worth 5807 points").unwrap());
    }

    #[test]
    fn part_two() {}
}
