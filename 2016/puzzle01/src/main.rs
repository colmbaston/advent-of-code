use std::collections::HashSet;

fn main()
{
    let input = include_str!("../input.txt").trim_end().split(", ").map(parse_inst).collect::<Vec<_>>();

    let mut x  : i32 = 0;
    let mut y  : i32 = 0;
    let mut dx : i32 = 0;
    let mut dy : i32 = 1;

    let mut visited  = HashSet::new();
    let mut location = None;

    for &(left, k) in input.iter()
    {
        std::mem::swap(&mut dx, &mut dy);
        if left { dx = -dx } else { dy = -dy }

        for _ in 0 .. k
        {
            if location.is_none() && !visited.insert((x, y))
            {
                location = Some((x, y))
            }

            x += dx;
            y += dy;
        }
    }
    println!("{}", x.abs() + y.abs());

    if let Some((x, y)) = location
    {
        println!("{}", x.abs() + y.abs());
    }
}

fn parse_inst(s : &str) -> (bool, i32)
{
    let (a, b) = s.split_at(1);
    (a == "L", b.parse().unwrap())
}
