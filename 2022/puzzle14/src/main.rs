use std::{ collections::HashSet, cmp::Ordering };

type Pos = (usize, usize);

fn main()
{
    let clay  = parse_clay(include_str!("../input.txt"));
    let max_y = 1 + clay.iter().map(|&(_, y)| y).max().unwrap_or(0);

    let mut grid = vec![vec![false ; max_y+1] ; 2*max_y+1];
    for (x, y) in clay.into_iter()
    {
        if let Some(x) = (x + max_y).checked_sub(500) { grid[x][y] = true }
    }

    let mut sand     = 0;
    let mut part_one = true;
    while let Some((x, y)) = drop_sand((max_y, 0), |(x, y)| y > max_y || grid[x][y])
    {
        if part_one && y == max_y
        {
            println!("{sand}");
            part_one = false;
        }
        grid[x][y] = true;
        sand += 1;
    }
    println!("{sand}");
}

fn drop_sand((sx, sy) : Pos, blocked : impl Fn(Pos) -> bool + Copy) -> Option<Pos>
{
    (sy ..).take_while(|y| !blocked((sx, *y)))
           .last().map(|y| sx.checked_sub(1).and_then(|x| drop_sand((x,    y+1), blocked))
                                            .or_else(||   drop_sand((sx+1, y+1), blocked))
                                            .unwrap_or((sx, y)))
}

fn parse_clay(s : &str) -> HashSet<Pos>
{
    let mut buff = Vec::new();
    let mut clay = HashSet::new();

    for line in s.lines()
    {
        buff.clear();
        buff.extend(line.split(" -> ").filter_map(|word|
        {
            let mut digits = word.split(',');
            digits.next().and_then(|x| x.parse::<usize>().ok())
                         .and_then(|x| digits.next().and_then(|y| y.parse::<usize>().ok().map(|y| (x, y))))
        }));

        for w in buff.windows(2)
        {
            let (ax, ay) = w[0];
            let (bx, by) = w[1];

            match (ax.cmp(&bx), ay.cmp(&by))
            {
                (Ordering::Equal, Ordering::Less ) => clay.extend((ay ..= by).map(|y| (ax, y))),
                (Ordering::Equal, _              ) => clay.extend((by ..= ay).map(|y| (ax, y))),
                (Ordering::Less,  Ordering::Equal) => clay.extend((ax ..= bx).map(|x| (x, ay))),
                (_,               Ordering::Equal) => clay.extend((bx ..= ax).map(|x| (x, ay))),
                _                                  => unreachable!()
            }
        }
    }

    clay
}
