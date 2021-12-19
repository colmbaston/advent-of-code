mod point;
use point::{ Point, Rotation };
use std::collections::HashSet;

fn main()
{
    let input = include_str!("../input.txt").split("\n\n")
                                            .map(|s| s.lines().skip(1).map(Point::parse).collect())
                                            .collect::<Vec<Vec<Point>>>();

    let (scanners, beacons) = reconstruct_map(0, &mut HashSet::new(), &input);
    println!("{}", beacons.len());

    let mut max = i32::MIN;
    for (i, ps) in scanners.iter().enumerate()
    {
        for pt in scanners.iter().skip(i+1)
        {
            max = max.max(ps.manhattan(pt));
        }
    }
    println!("{}", max);
}

fn intersect(a : &[Point], b : &[Point]) -> Option<(Rotation, Point)>
{
    const THRESHOLD : usize = 12;

    for rot in Point::rotations()
    {
        let b = b.iter().map(|&p| p.rotate(rot)).collect::<Vec<Point>>();

        for pa in a.iter()
        {
            for pb in b.iter()
            {
                let offset = pa - pb;
                if THRESHOLD <= b.iter().filter(|&pt| a.contains(&(pt + &offset))).count()
                {
                    return Some((rot, offset))
                }
            }
        }
    }

    None
}

fn reconstruct_map(i : usize, visited : &mut HashSet<usize>, input : &[Vec<Point>]) -> (Vec<Point>, HashSet<Point>)
{
    let mut scanners = vec![Point { x: 0, y: 0, z: 0 }];
    let mut beacons  = input[i].iter().cloned().collect::<HashSet<Point>>();

    visited.insert(i);
    for j in 0 .. input.len()
    {
        if visited.contains(&j) { continue }
        if let Some((rot, offset)) = intersect(&input[i], &input[j])
        {
            let (a, b) = reconstruct_map(j, visited, input);

            scanners.extend(a.into_iter().map(|p| &p.rotate(rot) + &offset));
            beacons.extend(b.into_iter().map(|p|  &p.rotate(rot) + &offset));
        }
    }

    (scanners, beacons)
}
