use crate::Coords::*;
use std::cmp::Ordering;

pub enum Coords
{
    Empty,
    Single(i64, i64),
    FixedX(i64, i64, i64),
    FixedY(i64, i64, i64)
}

impl Iterator for Coords
{
    type Item = (i64, i64);

    fn next(&mut self) -> Option<(i64, i64)>
    {
        match *self
        {
            Empty           => None,
            Single(x,y)     => { *self = Empty; Some((x,y)) },
            FixedX(x,y1,y2) =>
            {
                match y1.cmp(&y2)
                {
                    Ordering::Less    => *self = FixedX(x, y1+1, y2),
                    Ordering::Equal   => *self = Empty,
                    Ordering::Greater => *self = FixedX(x, y1-1, y2)
                }
                Some((x, y1))
            }
            FixedY(y,x1,x2) =>
            {
                match x1.cmp(&x2)
                {
                    Ordering::Less    => *self = FixedY(y, x1+1, x2),
                    Ordering::Equal   => *self = Empty,
                    Ordering::Greater => *self = FixedY(y, x1-1, x2)
                }
                Some((x1, y))
            }
        }
    }
}
