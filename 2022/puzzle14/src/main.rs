type Pos = (usize, usize);

fn main()
{
    let walls = include_str!("../input.txt").lines()
                                            .map(|l| parse_wall(l).collect::<Vec<Pos>>())
                                            .collect::<Vec<Vec<Pos>>>();

    let max_y = 1 + walls.iter()
                         .flat_map(|l| l.iter().map(|(_, y)| y))
                         .copied().max()
                         .unwrap_or(0);

    let mut grid = vec![vec![true ; max_y+1] ; 2*max_y+1];
    for wall in walls
    {
        for &[a, b] in wall.array_windows()
        {
            let (ax, ay) = a;
            let (bx, by) = b;

            if ax == bx
            {
                if let Some(x) = (ax + max_y).checked_sub(500)
                {
                    (ay.min(by) ..= ay.max(by)).for_each(|y| grid[x][y] = false);
                }
            }
            else
            {
                (ax.min(bx) ..= ax.max(bx)).filter_map(|x| (x + max_y).checked_sub(500))
                                           .for_each(|x| grid[x][ay] = false);
            }
        }
    }

    let mut sand     = 0;
    let mut part_one = true;
    while let Some((x, y)) = drop_sand((max_y, 0), |(x, y)| y <= max_y && grid[x][y])
    {
        if part_one && y == max_y
        {
            println!("{sand}");
            part_one = false;
        }
        grid[x][y] = false;
        sand += 1;
    }
    println!("{sand}");
}

fn drop_sand((sx, sy) : Pos, vacant : impl Fn(Pos) -> bool + Copy) -> Option<Pos>
{
    (sy ..).take_while(|y| vacant((sx, *y)))
           .last()
           .map(|y| sx.checked_sub(1).and_then(|x| drop_sand((x,    y+1), vacant))
                                     .or_else(||   drop_sand((sx+1, y+1), vacant))
                                     .unwrap_or((sx, y)))
}

fn parse_wall(s : &str) -> impl Iterator<Item = Pos> + '_
{
    s.split(" -> ").filter_map(|corner|
    {
        let mut k = corner.split(',');
        k.next().and_then(|x| x.parse::<usize>().ok())
                .and_then(|x| k.next().and_then(|y| y.parse::<usize>().ok().map(|y| (x, y))))
    })
}
