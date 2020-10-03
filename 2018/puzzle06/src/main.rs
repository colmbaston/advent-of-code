use std::cmp::Ordering;
use std::collections::HashMap;

fn main()
{
    let input = include_str!("../input.txt").lines().map(|s|
    {
        let mut i = s.split(", ").map(|t| t.parse::<i32>().unwrap());
        (i.next().unwrap(), i.next().unwrap())
    })
    .collect::<Vec<_>>();

    // determine the corners of the rectangle bounding all of the points
    let (min_x, min_y, max_x, max_y) = input.iter().fold((i32::MAX, i32::MAX, i32::MIN, i32::MIN), |(min_x, min_y, max_x, max_y), &(x, y)|
    {
        (min_x.min(x), min_y.min(y), max_x.max(x), max_y.max(y))
    });

    // loop over all points in the bounding rectangle excluding the boundary itself
    let mut count = 0;
    let mut areas = HashMap::new();
    for x in min_x+1 .. max_x
    {
        for y in min_y+1 .. max_y
        {
            // for part 1, find the size of the area nearest each point
            if let Some(n) = nearest(input.iter(), &(x, y))
            {
                *areas.entry(n).or_insert(0) += 1;
            }

            // for part 2, count those points where the manhattan distances sum to < 10_000
            if input.iter().map(|p| manhattan(p, &(x, y))).sum::<u32>() < 10_000
            {
                count += 1
            }
        }
    }

    // loop over the boundary of the rectangle
    let top    = (min_x   ..  max_x).map(|x| (x, min_y));
    let bottom = (min_x   ..  max_x).map(|x| (x, max_y));
    let left   = (min_y+1 ..  max_y).map(|y| (min_x, y));
    let right  = (min_y   ..= max_y).map(|y| (max_x, y));
    for point in top.chain(bottom).chain(left).chain(right)
    {
        // sanity check for part 2 as I'm assuming all
        // counted points will be inside the boundary
        debug_assert!(input.iter().map(|p| manhattan(p, &point)).sum::<u32>() >= 10_000);

        // remove those points from the area map which have nearest
        // points along the boundary as those areas will be infinite
        if let Some(n) = nearest(input.iter(), &point)
        {
            areas.remove(&n);
        }
    }

    println!("{}", areas.values().max().unwrap());
    println!("{}", count);
}

#[inline]
fn manhattan((x1, y1) : &(i32, i32), (x2, y2) : &(i32, i32)) -> u32
{
    ((x1 - x2).abs() + (y1 - y2).abs()) as u32
}

fn nearest<'a>(points : impl Iterator<Item = &'a (i32, i32)>, p1 : &(i32, i32)) -> Option<(i32, i32)>
{
    let mut near = Err(u32::MAX);
    for p2 in points
    {
        let d = manhattan(p1, p2);
        match near
        {
            Ok((_, k)) => match d.cmp(&k)
            {
                Ordering::Less    => near = Ok((*p2, d)),
                Ordering::Equal   => near = Err(d),
                Ordering::Greater => ()
            },
            Err(k) => if d < k { near = Ok((*p2, d)) }
        }
    }
    near.ok().map(|(c, _)| c)
}
