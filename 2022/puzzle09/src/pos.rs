use std::ops::{ Sub, Index, IndexMut };

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Pos { x: i16, y: i16 }

impl Pos
{
    pub const ORIGIN : Pos = Pos { x: 0, y: 0 };

    pub fn step_towards(self, leader : Pos) -> Option<Pos>
    {
        let diff = leader - self;
        (diff[Axis::X].abs() > 1 || diff[Axis::Y].abs() > 1).then(|| Pos
        {
            x: self.x + diff[Axis::X].signum(),
            y: self.y + diff[Axis::Y].signum()
        })
    }
}

impl Sub for Pos
{
    type Output = Pos;

    fn sub(self, other : Pos) -> Pos
    {
        Pos
        {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

#[derive(Clone, Copy)]
pub enum Axis { X, Y }

impl Index<Axis> for Pos
{
    type Output = i16;

    fn index(&self, axis : Axis) -> &i16
    {
        match axis
        {
            Axis::X => &self.x,
            Axis::Y => &self.y
        }
    }
}

impl IndexMut<Axis> for Pos
{
    fn index_mut(&mut self, axis : Axis) -> &mut i16
    {
        match axis
        {
            Axis::X => &mut self.x,
            Axis::Y => &mut self.y
        }
    }
}

pub struct Offset(pub Axis, pub i16);

impl Offset
{
    pub fn parse(s : &str) -> Option<Offset>
    {
        match s.as_bytes().first().and_then(|b| s[2 ..].parse::<i16>().ok().map(|k| (b, k)))
        {
            Some((b'R', k)) => Some(Offset(Axis::X,  k)),
            Some((b'L', k)) => Some(Offset(Axis::X, -k)),
            Some((b'U', k)) => Some(Offset(Axis::Y,  k)),
            Some((b'D', k)) => Some(Offset(Axis::Y, -k)),
            _               => None
        }
    }
}
