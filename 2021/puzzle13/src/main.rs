use std::collections::HashSet;

fn main()
{
    let (mut points, folds) = parse_instructions(include_str!("../input.txt"));
    let  mut next           = HashSet::new();

    for (i, (fx, k)) in folds.into_iter().enumerate()
    {
        for (x, y) in points.drain()
        {
            next.insert((if  fx && x > k { k+k-x } else { x },
                         if !fx && y > k { k+k-y } else { y }));
        }
        std::mem::swap(&mut points, &mut next);

        if i == 0 { println!("{}", points.len()) }
    }

    if let Some((min_x, min_y, max_x, max_y)) = aoc::bounds::bounds_2d(points.iter())
    {
        for y in min_y ..= max_y
        {
            for x in min_x ..= max_x
            {
                print!("{}", if points.contains(&(x, y)) { '#' } else { ' ' });
            }
            println!();
        }
    }
}

type Grid = HashSet<(i32, i32)>;
type Inst = (bool, i32);

fn parse_instructions(s : &str) -> (Grid, Vec<Inst>)
{
    let mut i  = s.split("\n\n");
    let points = i.next().unwrap().lines().map(|t|
    {
        let mut j = t.split(',');

        (j.next().unwrap().parse().unwrap(),
         j.next().unwrap().parse().unwrap())
    });

    let folds = i.next().unwrap().lines().map(|t|
    {
        let mut j = t.split('=');

        (j.next().unwrap().as_bytes().last().unwrap() == &b'x',
         j.next().unwrap().parse().unwrap())
    });

    (points.collect(), folds.collect())
}
