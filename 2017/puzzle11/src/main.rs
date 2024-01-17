use std::ops::AddAssign;

fn main()
{
    let mut max = 0;
    let mut pos = Axial::ORIGIN;
    for dir in include_str!("../input.txt").trim_end().split(',').map(Axial::parse)
    {
        pos += dir;
        max = max.max(pos.distance(Axial::ORIGIN));
    }
    println!("{}", pos.distance(Axial::ORIGIN));
    println!("{max}")
}

#[derive(Copy, Clone)]
struct Axial
{
    x: i32,
    y: i32
}

impl Axial
{
    const ORIGIN : Axial = Axial { x: 0, y: 0 };

    fn parse(s : &str) -> Axial
    {
        match s
        {
            "n"  => Axial { x:  0, y: -1 },
            "ne" => Axial { x:  1, y: -1 },
            "se" => Axial { x:  1, y:  0 },
            "s"  => Axial { x:  0, y:  1 },
            "sw" => Axial { x: -1, y:  1 },
            "nw" => Axial { x: -1, y:  0 },
            _    => unreachable!()
        }
    }

    fn distance(self, other : Axial) -> u32
    {
       (self.x.abs_diff(other.x) + (self.x + self.y).abs_diff(other.x + other.y) + self.y.abs_diff(other.y)) / 2
    }
}

impl AddAssign for Axial
{
    fn add_assign(&mut self, other : Axial)
    {
        self.x += other.x;
        self.y += other.y
    }
}
