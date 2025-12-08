fn main()
{
    let points = include_str!("../input.txt").lines().map(Point::parse).collect::<Vec<Point>>();

    let mut dists = Vec::new();
    for (i, &p) in points.iter().enumerate()
    {
        for (j, &q) in points.iter().enumerate().skip(i+1)
        {
            dists.push((p.distance_sq(q), i, j));
        }
    }
    dists.sort_unstable_by_key(|(d, _, _)| *d);

    let mut reps  = (0 .. points.len()).collect::<Vec<usize>>();
    let mut sizes = std::iter::repeat_n(1, points.len()).collect::<Vec<u32>>();
    for (k, (_, p, q)) in dists.into_iter().enumerate()
    {
        if k == 1000
        {
            let mut max = Vec::new();
            for &size in sizes.iter()
            {
                match max.iter().position(|&s| s < size)
                {
                    None    => if max.len() < 3 { max.push(size) },
                    Some(i) => { max.insert(i, size); max.truncate(3) }
                }
            }
            println!("{}", max.into_iter().product::<u32>());
        }

        let mut rp = p;
        loop
        {
            let grandparent = reps[reps[rp]];
            if rp == grandparent { break }
            reps[rp] = grandparent;
            rp       = grandparent;
        }

        let mut rq = q;
        loop
        {
            let grandparent = reps[reps[rq]];
            if rq == grandparent { break }
            reps[rq] = grandparent;
            rq       = grandparent;
        }

        if rp != rq
        {
            reps[rq]   = rp;
            sizes[rp] += std::mem::take(&mut sizes[rq]);

            if sizes[rp] == points.len() as u32
            {
                println!("{}", points[p].x * points[q].x);
                break
            }
        }
    }
}

#[derive(Copy, Clone)]
struct Point { x: u64, y: u64, z: u64 }

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

    fn distance_sq(self, other : Point) -> u64
    {
        self.x.abs_diff(other.x).pow(2) +
        self.y.abs_diff(other.y).pow(2) +
        self.z.abs_diff(other.z).pow(2)
    }
}
