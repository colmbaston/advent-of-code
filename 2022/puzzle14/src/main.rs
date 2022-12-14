use std::{ collections::HashSet, cmp::Ordering };

type Pos = (i16, i16);

fn main()
{
    let mut blocked  = parse_clay(include_str!("../input.txt"));
    let clay_len     = blocked.len();
    let max_y        = 1 + blocked.iter().map(|&(_, y)| y).max().unwrap_or(0);
    let mut part_one = true;
    while let Some(pos@(_, y)) = drop_sand((500, 0), |pos@(_, y)| y > max_y || blocked.contains(&pos))
    {
        if part_one && y == max_y
        {
            println!("{}", blocked.len() - clay_len);
            part_one = false;
        }
        blocked.insert(pos);
    }
    println!("{}", blocked.len() - clay_len);
}

fn drop_sand((sx, sy) : Pos, blocked : impl Fn(Pos) -> bool + Copy) -> Option<Pos>
{
    (sy ..).take_while(|y| !blocked((sx, *y)))
           .last().map(|y|            drop_sand((sx-1, y+1), blocked)
                          .or_else(|| drop_sand((sx+1, y+1), blocked))
                          .unwrap_or((sx, y)))
}

fn parse_clay(s : &str) -> HashSet<Pos>
{
    let mut buff  = Vec::new();
    let mut walls = HashSet::new();

    for line in s.lines()
    {
        buff.clear();
        buff.extend(line.split(" -> ").filter_map(|word|
        {
            let mut digits = word.split(',');
            digits.next().and_then(|x| x.parse::<i16>().ok())
                         .and_then(|x| digits.next().and_then(|y| y.parse::<i16>().ok().map(|y| (x, y))))
        }));

        for w in buff.windows(2)
        {
            let (ax, ay) = w[0];
            let (bx, by) = w[1];

            match (ax.cmp(&bx), ay.cmp(&by))
            {
                (Ordering::Equal, Ordering::Less ) => walls.extend((ay ..= by).map(|y| (ax, y))),
                (Ordering::Equal, _              ) => walls.extend((by ..= ay).map(|y| (ax, y))),
                (Ordering::Less,  Ordering::Equal) => walls.extend((ax ..= bx).map(|x| (x, ay))),
                (_,               Ordering::Equal) => walls.extend((bx ..= ax).map(|x| (x, ay))),
                _                                  => unreachable!()
            }
        }
    }

    walls
}
