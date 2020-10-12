use std::collections::HashSet;

fn main()
{
    let mut points = include_str!("../input.txt").lines().map(parse).collect::<HashSet<Point4D>>();
    let mut queue  = Vec::new();
    let mut count  = 0;

    while !points.is_empty()
    {
        // the set is not empty, so choose an arbitrary member
        // and queue its constellation for removal from the set
        queue.push(*points.iter().next().unwrap());

        // purge the chosen element's constellation
        while let Some(point) = queue.pop()
        {
            if points.remove(&point)
            {
                queue.extend(points.iter().copied().filter(|&other| manhattan(point, other) <= 3))
            }
        }

        count += 1;
    }

    println!("{}", count);
}

type Point4D = (i32, i32, i32, i32);

fn parse(s : &str) -> Point4D
{
    let mut it = s.split(',');

    let w = it.next().unwrap().parse().unwrap();
    let x = it.next().unwrap().parse().unwrap();
    let y = it.next().unwrap().parse().unwrap();
    let z = it.next().unwrap().parse().unwrap();

    (w, x, y, z)
}

fn manhattan((w1, x1, y1, z1) : Point4D, (w2, x2, y2, z2) : Point4D) -> u32
{
    ((w1 - w2).abs() + (x1 - x2).abs() + (y1 - y2).abs() + (z1 - z2).abs()) as u32
}
