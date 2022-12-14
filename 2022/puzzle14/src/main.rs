use std::{ collections::HashSet, cmp::Ordering };

type Pos = (u32, u32);
const SOURCE : Pos = (500, 0);

fn main()
{
    let walls = parse_walls(include_str!("../input.txt"));
    let max_y = 1 + walls.iter().map(|(_, y)| y).copied().max().unwrap_or(0);

    let mut sand     = HashSet::new();
    let mut part_one = true;
    while let Some(pos@(_, y)) = drop_sand(SOURCE, max_y, |p| walls.contains(p) || sand.contains(p))
    {
        if part_one && y == max_y
        {
            println!("{}", sand.len());
            part_one = false;
        }
        sand.insert(pos);
    }
    println!("{}", sand.len());
}

fn drop_sand((sx, sy) : Pos, max_y : u32, blocked : impl Fn(&Pos) -> bool + Copy) -> Option<Pos>
{
    (sy ..).map(|y| (sx, y))
           .take_while(|p| p.1 <= max_y && !blocked(p))
           .last().and_then(|(x, y)| drop_sand((x-1, y+1), max_y, blocked)
                         .or_else(|| drop_sand((x+1, y+1), max_y, blocked))
                         .or(Some((x, y))))
}

fn parse_walls(s : &str) -> HashSet<Pos>
{
    let mut buff  = Vec::new();
    let mut walls = HashSet::new();

    s.lines().map(|line| line.split(" -> ").filter_map(|word|
    {
        let mut digits = word.split(',');
        digits.next().and_then(|x| x.parse::<u32>().ok())
                     .and_then(|x| digits.next().and_then(|y| y.parse::<u32>().ok().map(|y| (x, y))))
    }))
    .for_each(|line|
    {
        buff.clear();
        buff.extend(line);
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
    });

    walls
}
