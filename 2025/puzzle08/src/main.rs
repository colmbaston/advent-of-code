use std::collections::{ HashMap, HashSet };

fn main()
{
    let points    = include_str!("../input.txt").lines().map(Point::parse).collect::<Vec<Point>>();
    let mut dists = Vec::new();
    for (i, &p) in points.iter().enumerate()
    {
        for &q in points[i+1 ..].iter()
        {
            dists.push((p.distance(q), p, q));
        }
    }
    dists.sort_unstable_by(|d, e| d.0.total_cmp(&e.0));

    let mut reps     = points.iter().map(|&p| (p, p)).collect::<HashMap<Point, Point>>();
    let mut circuits = points.iter().map(|&p| (p, std::iter::once(p).collect())).collect::<HashMap<Point, HashSet<Point>>>();
    for (i, &(_, p, q)) in dists.iter().enumerate()
    {
        if i == 1000
        {
            let mut sizes = Vec::new();
            for (rp, rq) in reps.iter()
            {
                if rp != rq { continue }
                sizes.push(circuits[rp].len() as u64)
            }
            sizes.sort_unstable();
            println!("{}", sizes.into_iter().rev().take(3).product::<u64>());
        }

        let rp = reps[&p];
        let rq = reps[&q];
        if rp == rq { continue }

        let union = circuits[&rp].union(&circuits[&rq]).copied().collect::<HashSet<Point>>();
        for &r in union.iter()
        {
            reps.insert(r, rp);
            circuits.remove(&r);
        }
        circuits.insert(rp, union);

        if circuits.len() == 1
        {
            println!("{}", p.x * q.x);
            break
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Point { x: i64, y: i64, z: i64 }

impl Point
{
    fn parse(s : &str) -> Point
    {
        let mut sep = s.split(',');
        Point
        {
            x: sep.next().unwrap().parse().unwrap(),
            y: sep.next().unwrap().parse().unwrap(),
            z: sep.next().unwrap().parse().unwrap()
        }
    }

    fn distance(self, other : Point) -> f32
    {
        (((self.x-other.x).pow(2) +
          (self.y-other.y).pow(2) +
          (self.z-other.z).pow(2)) as f32).sqrt()
    }
}
