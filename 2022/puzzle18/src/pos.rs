use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos
{
    pub x: i32,
    pub y: i32,
    pub z: i32
}

impl Pos
{
    pub fn parse(s : &str) -> Option<Pos>
    {
        let mut parts = s.split(',').map(|p| p.parse().ok());
        Some(Pos { x: parts.next()??, y: parts.next()??, z: parts.next()?? })
    }

    pub fn adjacents(self) -> impl Iterator<Item = Pos>
    {
        let Pos { x, y, z } = self;

        [Pos { x: x+1, y,      z      },
         Pos { x: x-1, y,      z      },
         Pos { x,      y: y+1, z      },
         Pos { x,      y: y-1, z      },
         Pos { x,      y,      z: z+1 },
         Pos { x,      y,      z: z-1 }].into_iter()
    }

    pub fn min_corner(self, other : Pos) -> Pos
    {
        Pos
        {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
            z: self.z.min(other.z)
        }
    }

    pub fn max_corner(self, other : Pos) -> Pos
    {
        Pos
        {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
            z: self.z.max(other.z)
        }
    }

    pub fn in_rect(self, min : Pos, max : Pos) -> bool
    {
        (min.x ..= max.x).contains(&self.x) &&
        (min.y ..= max.y).contains(&self.y) &&
        (min.z ..= max.z).contains(&self.z)
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
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
