#![feature(iter_next_chunk)]
use std::ops::RangeInclusive;

fn main()
{
    let hailstones = include_str!("../input.txt").lines().map(Hailstone::parse).collect::<Vec<Hailstone>>();

    let mut count = 0;
    for (i, ha) in hailstones.iter().enumerate()
    {
        for hb in hailstones[i+1..].iter()
        {
            if let Some((x, y)) = ha.intersect_2d(hb)
            {
                const TEST_RANGE : RangeInclusive<f64> = 200_000_000_000_000.0
                                                     ..= 400_000_000_000_000.0;

                if TEST_RANGE.contains(&x)
                && TEST_RANGE.contains(&y) { count += 1 }
            }
        }
    }
    println!("{count}");
}

struct Hailstone
{
    pos: Vec3,
    vel: Vec3
}

struct Vec3
{
    x: i64,
    y: i64,
    z: i64
}

impl Hailstone
{
    fn parse(s : &str) -> Hailstone
    {
        let (a, b) = s.split_once('@').unwrap();
        Hailstone { pos: Vec3::parse(a), vel: Vec3::parse(b) }
    }

    fn intersect_2d(&self, other : &Hailstone) -> Option<(f64, f64)>
    {
        let Vec3 { x: pax, y: pay, .. } = self.pos;
        let Vec3 { x: vax, y: vay, .. } = self.vel;
        let Vec3 { x: pbx, y: pby, .. } = other.pos;
        let Vec3 { x: vbx, y: vby, .. } = other.vel;

        let det = vbx * vay - vax * vby;
        if let 0 = det { return None }

        let ta = (vbx * (pby - pay) - vby * (pbx - pax)) as f64 / det as f64;
        let tb = (vax * (pby - pay) - vay * (pbx - pax)) as f64 / det as f64;

        (ta >= 0.0 && tb >= 0.0).then_some((pax as f64 + vax as f64 * ta,
                                            pay as f64 + vay as f64 * ta))
    }
}

impl Vec3
{
    fn parse(s : &str) -> Vec3
    {
        let [x, y, z] = s.split(',')
                         .map(|k| k.trim().parse().unwrap())
                         .next_chunk().unwrap();

        Vec3 { x, y, z }
    }
}
