use std::ops::{ Add, Sub };

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point
{
    x: i32,
    y: i32,
    z: i32
}

#[derive(Clone, Copy)]
pub struct Rotation(u8);

impl Add for &Point
{
    type Output = Point;

    fn add(self, other : &Point) -> Point
    {
        Point
        {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Sub for &Point
{
    type Output = Point;

    fn sub(self, other : &Point) -> Point
    {
        Point
        {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl Point
{
    pub fn parse(s : &str) -> Point
    {
        let mut i = s.split(',');

        Point
        {
            x: i.next().unwrap().parse().unwrap(),
            y: i.next().unwrap().parse().unwrap(),
            z: i.next().unwrap().parse().unwrap()
        }
    }

    pub fn origin() -> Point
    {
        Point { x: 0, y: 0, z: 0 }
    }

    pub fn manhattan(&self, other : &Point) -> i32
    {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }

    pub fn rotate(&self, Rotation(r) : Rotation) -> Point
    {
        match r
        {
             0 => Point { x:  self.x, y:  self.y, z:  self.z },
             1 => Point { x: -self.x, y: -self.y, z:  self.z },
             2 => Point { x: -self.x, y:  self.y, z: -self.z },
             3 => Point { x:  self.x, y: -self.y, z: -self.z },

             4 => Point { x:  self.z, y:  self.x, z:  self.y },
             5 => Point { x: -self.z, y: -self.x, z:  self.y },
             6 => Point { x: -self.z, y:  self.x, z: -self.y },
             7 => Point { x:  self.z, y: -self.x, z: -self.y },

             8 => Point { x:  self.y, y:  self.z, z:  self.x },
             9 => Point { x: -self.y, y: -self.z, z:  self.x },
            10 => Point { x: -self.y, y:  self.z, z: -self.x },
            11 => Point { x:  self.y, y: -self.z, z: -self.x },

            12 => Point { x: -self.z, y:  self.y, z:  self.x },
            13 => Point { x:  self.z, y: -self.y, z:  self.x },
            14 => Point { x:  self.z, y:  self.y, z: -self.x },
            15 => Point { x: -self.z, y: -self.y, z: -self.x },

            16 => Point { x: -self.x, y:  self.z, z:  self.y },
            17 => Point { x:  self.x, y: -self.z, z:  self.y },
            18 => Point { x:  self.x, y:  self.z, z: -self.y },
            19 => Point { x: -self.x, y: -self.z, z: -self.y },

            20 => Point { x: -self.y, y:  self.x, z:  self.z },
            21 => Point { x:  self.y, y: -self.x, z:  self.z },
            22 => Point { x:  self.y, y:  self.x, z: -self.z },
            23 => Point { x: -self.y, y: -self.x, z: -self.z },

            _  => unreachable!()
        }
    }

    pub fn rotations() -> impl Iterator<Item = Rotation>
    {
        (0 .. 24).map(Rotation)
    }
}
