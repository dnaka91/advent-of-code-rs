use std::convert::TryFrom;

use anyhow::{bail, Result};
use derive_more::{Add, Display, Div, Mul, Sub};
use num_traits::PrimInt;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Add, Sub, Mul, Div, Display)]
#[display(fmt = "({}, {})", _0, _1)]
pub struct Point<T: PrimInt>(pub T, pub T);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Display)]
pub enum BasicLine<T: PrimInt> {
    #[display(fmt = "(({}, {}), ({}, {}))", _1, _0, _2, _0)]
    HorzLine(T, T, T),
    #[display(fmt = "(({}, {}), ({}, {}))", _0, _1, _0, _2)]
    VertLine(T, T, T),
}

impl<T: PrimInt> BasicLine<T> {}

impl<T: PrimInt> TryFrom<(Point<T>, Point<T>)> for BasicLine<T> {
    type Error = anyhow::Error;

    fn try_from(value: (Point<T>, Point<T>)) -> Result<Self> {
        Ok(match value {
            (Point(x1, y1), Point(x2, y2)) => {
                if x1 == x2 {
                    Self::VertLine(x1, y1, y2)
                } else if y1 == y2 {
                    Self::HorzLine(y1, x1, x2)
                } else {
                    bail!("Only supporting horizontal or vertical lines")
                }
            }
        })
    }
}

impl<T: PrimInt> Line<T> for BasicLine<T> {
    fn a(&self) -> Point<T> {
        match self {
            Self::HorzLine(y, x1, _) => Point(*x1, *y),
            Self::VertLine(x, y1, _) => Point(*x, *y1),
        }
    }

    fn b(&self) -> Point<T> {
        match self {
            Self::HorzLine(y, _, x2) => Point(*x2, *y),
            Self::VertLine(x, _, y2) => Point(*x, *y2),
        }
    }
}

trait Line<T: PrimInt> {
    fn a(&self) -> Point<T>;
    fn b(&self) -> Point<T>;
}

impl<T: PrimInt> std::ops::Add<Point<T>> for BasicLine<T> {
    type Output = Self;

    fn add(self, rhs: Point<T>) -> Self::Output {
        match self {
            Self::HorzLine(y, x1, x2) => Self::HorzLine(y + rhs.1, x1 + rhs.0, x2 + rhs.0),
            Self::VertLine(x, y1, y2) => Self::VertLine(x + rhs.0, y1 + rhs.1, y2 + rhs.1),
        }
    }
}

impl<T: PrimInt> std::ops::Sub<Point<T>> for BasicLine<T> {
    type Output = Self;

    fn sub(self, rhs: Point<T>) -> Self::Output {
        match self {
            Self::HorzLine(y, x1, x2) => Self::HorzLine(y - rhs.1, x1 - rhs.0, x2 - rhs.0),
            Self::VertLine(x, y1, y2) => Self::VertLine(x - rhs.0, y1 - rhs.1, y2 - rhs.1),
        }
    }
}

impl<T: PrimInt + std::ops::AddAssign> IntoIterator for BasicLine<T> {
    type Item = Point<T>;
    type IntoIter = BasicLineIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Self::HorzLine(_, x, _) => BasicLineIter { line: self, count: x },
            Self::VertLine(_, y, _) => BasicLineIter { line: self, count: y },
        }
    }
}

pub struct BasicLineIter<T: PrimInt> {
    line: BasicLine<T>,
    count: T,
}

impl<T: PrimInt + std::ops::AddAssign> Iterator for BasicLineIter<T> {
    type Item = Point<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += T::one();

        match self.line {
            BasicLine::HorzLine(y, _, x2) => {
                if self.count < x2 {
                    Some(Point(self.count, y))
                } else {
                    None
                }
            }
            BasicLine::VertLine(x, _, y2) => {
                if self.count < y2 {
                    Some(Point(x, self.count))
                } else {
                    None
                }
            }
        }
    }
}
