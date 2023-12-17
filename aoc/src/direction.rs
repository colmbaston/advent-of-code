use std::ops::{ Add, Sub };
use num_traits::{ CheckedAdd, CheckedSub, One };

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Direction { North, East, South, West }

impl Direction
{
    pub fn offset<T : Add<Output = T> + Sub<Output = T> + One>(self, (x, y) : (T, T), d : T) -> (T, T)
    {
        match self
        {
            Direction::North => (x, y - d),
            Direction::East  => (x + d, y),
            Direction::South => (x, y + d),
            Direction::West  => (x - d, y)
        }
    }

    pub fn checked_offset<T : CheckedAdd + CheckedSub + One>(self, (x, y) : (T, T), d : T) -> Option<(T, T)>
    {
        match self
        {
            Direction::North => y.checked_sub(&d).map(|y| (x, y)),
            Direction::East  => x.checked_add(&d).map(|x| (x, y)),
            Direction::South => y.checked_add(&d).map(|y| (x, y)),
            Direction::West  => x.checked_sub(&d).map(|x| (x, y))
        }
    }

    pub fn step<T : Add<Output = T> + Sub<Output = T> + One>(self, pos : (T, T)) -> (T, T)
    {
        self.offset(pos, T::one())
    }

    pub fn checked_step<T : CheckedAdd + CheckedSub + One>(self, pos : (T, T)) -> Option<(T, T)>
    {
        self.checked_offset(pos, T::one())
    }

    pub fn clockwise(self) -> Direction
    {
        match self
        {
            Direction::North => Direction::East,
            Direction::East  => Direction::South,
            Direction::South => Direction::West,
            Direction::West  => Direction::North
        }
    }

    pub fn opposite(self) -> Direction
    {
        self.clockwise().clockwise()
    }

    pub fn anticlockwise(self) -> Direction
    {
        self.clockwise().opposite()
    }

    pub fn reflect(self) -> Direction
    {
        match self
        {
            Direction::North => Direction::East,
            Direction::East  => Direction::North,
            Direction::South => Direction::West,
            Direction::West  => Direction::South
        }
    }

    pub fn bit(self) -> u8
    {
        1 << self as u8
    }
}
