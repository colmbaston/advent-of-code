use std::collections::HashMap;

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
    dists.sort_unstable_by(|a, b| a.0.total_cmp(&b.0));

    let mut reps     = points.iter().map(|&p| (p, p)).collect::<HashMap<Point, Point>>();
    let mut circuits = points.iter().map(|&p| (p, vec![p])).collect::<HashMap<Point, Vec<Point>>>();
    for (i, &(_, p, q)) in dists.iter().enumerate()
    {
        if i == 1000
        {
            let mut sizes = Vec::new();
            for (rp, rq) in reps.iter()
            {
                if rp != rq { continue }
                sizes.push(circuits[rp].len() as u32)
            }
            sizes.sort_unstable();
            println!("{}", sizes.into_iter().rev().take(3).product::<u32>());
        }

        let rp = reps[&p];
        let rq = reps[&q];
        if rp == rq { continue }

        let circuit = circuits.remove(&rq).unwrap();
        for &r in circuit.iter()
        {
            reps.insert(r, rp);
        }
        circuits.entry(rp).and_modify(|c| c.extend(circuit));

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
