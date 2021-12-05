use std::collections::HashMap;

fn main()
{
    let mut hv   = HashMap::new();
    let mut diag = HashMap::new();

    for (x1, y1, x2, y2) in include_str!("../input.txt").lines().map(parse_line)
    {
        write_points(x1, y1, x2, y2, if x1 == x2 || y1 == y2 { &mut hv } else { &mut diag })
    }
    println!("{}", hv.values().filter(|&v| *v > 1).count());

    for (k, v) in hv.into_iter()
    {
        *diag.entry(k).or_insert(0) += v;
    }
    println!("{}", diag.values().filter(|&v| *v > 1).count());
}

fn parse_line(s : &str) -> (i32, i32, i32, i32)
{
    let mut i  = s.split(" -> ");

    let mut j = i.next().unwrap().split(',');
    let x1    = j.next().unwrap().parse().unwrap();
    let y1    = j.next().unwrap().parse().unwrap();

    j         = i.next().unwrap().split(',');
    let x2    = j.next().unwrap().parse().unwrap();
    let y2    = j.next().unwrap().parse().unwrap();

    (x1, y1, x2, y2)
}

fn write_points(mut x1 : i32, mut y1 : i32, x2 : i32, y2 : i32, map : &mut HashMap<(i32, i32), u32>)
{
    let dx = (x2 - x1).signum();
    let dy = (y2 - y1).signum();

    loop
    {
        *map.entry((x1, y1)).or_insert(0) += 1;

        if x1 == x2 && y1 == y2 { break }

        x1 += dx;
        y1 += dy;
    }
}
