use std::collections::HashMap;

fn main()
{
    let     points = parse_points(include_str!("../input.txt"));
    let mut grid   = points.iter().map(|&p| (p, 0)).collect::<HashMap<Point, u8>>();

    for p in points.into_iter().flat_map(|p| adjacent(p))
    {
        if let Some(n) = grid.get_mut(&p) { *n += 1 }
    }
    println!("{}", grid.values().filter(|&n| *n < 4).count());

    let initial_len    = grid.len();
    let mut accessible = Vec::new();
    let mut changed    = true;
    while changed
    {
        for (&p, &n) in grid.iter()
        {
            if n < 4 { accessible.push(p); }
        }
        changed = !accessible.is_empty();

        while let Some(p) = accessible.pop()
        {
            if grid.remove(&p).is_some()
            {
                for q in adjacent(p)
                {
                    if let Some(n) = grid.get_mut(&q)
                    {
                        *n -= 1;
                        if *n < 4 { accessible.push(q) }
                    }
                }
            }
        }
    }
    println!("{}", initial_len - grid.len());
}

type Point = (i32, i32);

fn parse_points(s : &str) -> Vec<Point>
{
    let mut points = Vec::new();
    for (l, y) in s.lines().zip(0 ..)
    {
        for (b, x) in l.bytes().zip(0 ..)
        {
            if b == b'@' { points.push((x, y)) }
        }
    }
    points
}

fn adjacent((x, y) : Point) -> impl Iterator<Item = Point>
{
    [(x-1, y-1), (x, y-1), (x+1, y-1),
     (x-1, y  ),           (x+1, y  ),
     (x-1, y+1), (x, y+1), (x+1, y+1)].into_iter()
}
