use std::ops::RangeInclusive;

fn main()
{
    let input = include_str!("../input.txt").lines().map(Cuboid::parse).collect::<Vec<(bool, Cuboid)>>();

    println!("{}", reboot(input.iter().filter(|(_, c)| c.subregion(&Cuboid { x: -50 ..= 50, y: -50 ..= 50, z: -50 ..= 50 }))));
    println!("{}", reboot(input.iter()));
}

#[derive(Clone)]
struct Cuboid
{
    x: RangeInclusive<i64>,
    y: RangeInclusive<i64>,
    z: RangeInclusive<i64>
}

impl Cuboid
{
    fn parse(s : &str) -> (bool, Cuboid)
    {
        let mut i = s.split([' ', '=', ',', '.']).step_by(2);
        let on    = i.next().unwrap() == "on";
        let min_x = i.next().unwrap().parse().unwrap();
        let max_x = i.next().unwrap().parse().unwrap();
        let min_y = i.next().unwrap().parse().unwrap();
        let max_y = i.next().unwrap().parse().unwrap();
        let min_z = i.next().unwrap().parse().unwrap();
        let max_z = i.next().unwrap().parse().unwrap();

        (on, Cuboid { x: min_x ..= max_x, y: min_y ..= max_y, z: min_z ..= max_z })
    }
}

trait Region
{
    fn area(&self) -> i64;
    fn intersect(&self, other : &Self) -> Option<Self> where Self : Sized;
    fn subregion(&self, other : &Self) -> bool;
}

impl Region for RangeInclusive<i64>
{
    fn area(&self) -> i64
    {
        (1 + self.end() - self.start()).max(0)
    }

    fn intersect(&self, other : &RangeInclusive<i64>) -> Option<RangeInclusive<i64>>
    {
        let max_start = *self.start().max(other.start());
        let min_end   = *self.end().min(other.end());

        (max_start <= min_end).then_some(max_start ..= min_end)
    }

    fn subregion(&self, other : &RangeInclusive<i64>) -> bool
    {
        other.start() <= self.start() && self.end() <= other.end()
    }
}

impl Region for Cuboid
{
    fn area(&self) -> i64
    {
        self.x.area() * self.y.area() * self.z.area()
    }

    fn intersect(&self, other : &Cuboid) -> Option<Cuboid>
    {
        Some(Cuboid
        {
            x: self.x.intersect(&other.x)?,
            y: self.y.intersect(&other.y)?,
            z: self.z.intersect(&other.z)?
        })
    }

    fn subregion(&self, other : &Cuboid) -> bool
    {
        self.x.subregion(&other.x) && self.y.subregion(&other.y) && self.z.subregion(&other.z)
    }
}

fn reboot<'a>(it : impl Iterator<Item = &'a (bool, Cuboid)>) -> i64
{
    let mut cuboids   : Vec<(bool, Cuboid)> = Vec::new();
    let mut extension : Vec<(bool, Cuboid)> = Vec::new();

    for (on, current) in it
    {
        for (sign, prev) in cuboids.iter()
        {
            if let Some(region) = current.intersect(prev)
            {
                extension.push((!sign, region));
            }
        }

        cuboids.append(&mut extension);
        if *on { cuboids.push((true, current.clone())) }
    }

    cuboids.into_iter().map(|(sign, c)| if sign { c.area() } else { -c.area() }).sum()
}
