use std::collections::HashMap;

fn main()
{
    let mut hv   = HashMap::new();
    let mut diag = HashMap::new();
    for (x1, y1, x2, y2) in include_str!("../input.txt").lines().map(parse_line)
    {
        if x1 == x2
        {
            for y in y1.min(y2) ..= y1.max(y2)
            {
                *hv.entry((x1, y)).or_insert(0) += 1;
            }
        }
        else if y1 == y2
        {
            for x in x1.min(x2) ..= x1.max(x2)
            {
                *hv.entry((x, y1)).or_insert(0) += 1;
            }
        }
        else if x1 < x2
        {
            if y1 < y2
            {
                for (x, y) in (x1 ..= x2).zip(y1 ..= y2)
                {
                    *diag.entry((x, y)).or_insert(0) += 1;
                }
            }
            else
            {
                for (x, y) in (x1 ..= x2).zip((y2 ..= y1).rev())
                {
                    *diag.entry((x, y)).or_insert(0) += 1;
                }
            }
        }
        else
        {
            if y1 < y2
            {
                for (x, y) in ((x2 ..= x1).rev()).zip(y1 ..= y2)
                {
                    *diag.entry((x, y)).or_insert(0) += 1;
                }
            }
            else
            {
                for (x, y) in ((x2 ..= x1).rev()).zip((y2 ..= y1).rev())
                {
                    *diag.entry((x, y)).or_insert(0) += 1;
                }
            }
        }
    }
    println!("{}", hv.values().filter(|&v| *v > 1).count());

    for (k, v) in hv.into_iter()
    {
        *diag.entry(k).or_insert(0) += v;
    }
    println!("{}", diag.values().filter(|&v| *v > 1).count());
}

fn parse_line(s : &str) -> (u32, u32, u32, u32)
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
