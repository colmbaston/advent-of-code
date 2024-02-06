#![feature(iter_next_chunk)]
use std::ops::AddAssign;

fn main()
{
    let mut particles = include_str!("../input.txt").lines().map(Particle::parse).collect::<Vec<Particle>>();

    let min_acc        = particles.iter().map(|p| p.acc.manhattan()).min().unwrap();
    let mut candidates = particles.iter().enumerate()
                                  .filter(|(_, p)| p.acc.manhattan() == min_acc)
                                  .map(|(i, p)| (i, p.clone()))
                                  .collect::<Vec<(usize, Particle)>>();

    while candidates.iter().any(|(_, p)| !p.signum_match())
    {
        candidates.iter_mut().for_each(|(_, p)| p.step())
    }
    println!("{}", candidates.into_iter()
                             .min_by_key(|(_, p)| (p.acc.manhattan(), p.vel.manhattan(), p.pos.manhattan()))
                             .unwrap().0);

    let mut buffer = Vec::new();
    while particles.iter().any(|p| !p.signum_match())
    {
        particles.iter_mut().for_each(|p| p.step());
        particles.sort_unstable();

        buffer.clear();
        for group in particles.chunk_by(|a, b| a.pos == b.pos)
        {
            let (first, rest) = group.split_first().unwrap();
            if rest.is_empty() { buffer.push(first.clone()) }
        }
        std::mem::swap(&mut particles, &mut buffer);
    }
    println!("{}", particles.len());
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Particle
{
    pos: Vec3,
    vel: Vec3,
    acc: Vec3
}

impl Particle
{
    fn parse(s : &str) -> Particle
    {
        let [p, v, a] = s.split(", ").next_chunk().unwrap();

        Particle
        {
            pos: Vec3::parse(p.strip_prefix("p=").unwrap()),
            vel: Vec3::parse(v.strip_prefix("v=").unwrap()),
            acc: Vec3::parse(a.strip_prefix("a=").unwrap())
        }
    }

    fn step(&mut self)
    {
        self.vel += self.acc;
        self.pos += self.vel
    }

    fn signum_match(&self) -> bool
    {
        self.pos.signum_match(self.vel) &&
        self.pos.signum_match(self.acc)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Vec3
{
    x: i64,
    y: i64,
    z: i64
}

impl Vec3
{
    fn parse(s : &str) -> Vec3
    {
        let [x, y, z] = s.strip_prefix('<').unwrap()
                         .strip_suffix('>').unwrap()
                         .split(',')
                         .map(|k| k.parse().unwrap())
                         .next_chunk().unwrap();

        Vec3 { x, y, z }
    }

    fn signum_match(self, other : Vec3) -> bool
    {
        (self.x == 0 || other.x == 0 || self.x.signum() == other.x.signum()) &&
        (self.y == 0 || other.y == 0 || self.y.signum() == other.y.signum()) &&
        (self.z == 0 || other.z == 0 || self.z.signum() == other.z.signum())
    }

    fn manhattan(self) -> i64
    {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl AddAssign for Vec3
{
    fn add_assign(&mut self, other : Vec3)
    {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z
    }
}
