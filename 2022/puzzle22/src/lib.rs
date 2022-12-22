use std::{ ops::Add, collections::HashMap };

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pos
{
    pub x: i32,
    pub y: i32
}

impl Pos
{
    pub fn line(mut self, facing : Facing) -> impl Iterator<Item = Pos>
    {
        let offset = facing.offset();
        std::iter::from_fn(move || { self = self + offset; Some(self) })
    }
}

impl Add for Pos
{
    type Output = Pos;

    fn add(self, other : Pos) -> Pos
    {
        Pos
        {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

#[derive(Clone, Copy)]
pub enum Tile { Open, Wall }

impl Tile
{
    pub fn parse_grid(s : &str) -> HashMap<Pos, Tile>
    {
        s.lines().zip(1 ..)
         .flat_map(|(l, y)| l.bytes().zip(1 ..).filter_map(move |(b, x)| match b
         {
             b'.' => Some((Pos { x, y }, Tile::Open)),
             b'#' => Some((Pos { x, y }, Tile::Wall)),
             _    => None
         }))
         .collect()
    }
}

pub enum Inst { Move(u32), Turn(Turn) }

#[derive(Clone, Copy)]
pub enum Turn { Left, Right }

impl Inst
{
    pub fn parse(s : &str) -> Option<(Inst, &str)>
    {
        match s.as_bytes().first()?
        {
            b'0' ..= b'9' =>
            {
                let (digits, rest) = s.split_at(s.find(|c : char| !c.is_ascii_digit()).unwrap_or(s.len()));
                Some((Inst::Move(digits.parse().ok()?), rest))
            },
            b'L' => Some((Inst::Turn(Turn::Left),  &s[1 ..])),
            b'R' => Some((Inst::Turn(Turn::Right), &s[1 ..])),
            _    => None
        }
    }

    pub fn parse_path(mut s : &str) -> Vec<Inst>
    {
        let mut path = Vec::new();
        while let Some((inst, rest)) = Inst::parse(s)
        {
            path.push(inst);
            s = rest;
        }
        path
    }
}

#[derive(Clone, Copy)]
pub enum Facing { Right, Down, Left, Up }

impl From<u8> for Facing
{
    fn from(b : u8) -> Facing
    {
        match b % 4
        {
            0 => Facing::Right,
            1 => Facing::Down,
            2 => Facing::Left,
            3 => Facing::Up,
            _ => unreachable!()
        }
    }
}

impl Facing
{
    pub fn turn(self, turn : Turn) -> Facing
    {
        (self as u8 + match turn { Turn::Left => 3, Turn::Right => 1 }).into()
    }

    pub fn opposite(self) -> Facing
    {
        (self as u8 + 2).into()
    }

    pub fn offset(self) -> Pos
    {
        match self
        {
            Facing::Right => Pos { x:  1, y:  0 },
            Facing::Down  => Pos { x:  0, y:  1 },
            Facing::Left  => Pos { x: -1, y:  0 },
            Facing::Up    => Pos { x:  0, y: -1 }
        }
    }
}
