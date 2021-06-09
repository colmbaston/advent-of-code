use std::collections::HashSet;

fn main()
{
    let input    = include_str!("../input.txt");
    let mut grid = HashSet::new();
    let mut next = HashSet::new();

    for &p in [false, true].iter()
    {
        grid.clear();
        grid.extend(parse_grid(input));

        for _ in 0 .. 100
        {
            next.clear();

            for x in 0 .. 100
            {
                for y in 0 ..  100
                {
                    let k = neighbours(x, y).filter(|n| grid.contains(&n)).count();
                    if grid.contains(&(x, y)) && k == 2 || k == 3 { next.insert((x, y)); }
                }
            }

            if p { next.extend([(0, 0), (99, 0), (0, 99), (99, 99)].iter().cloned()) }
            std::mem::swap(&mut grid, &mut next);
        }
        println!("{}", grid.len());
    }
}

fn parse_grid(s : &str) -> impl '_ + Iterator<Item = (i8, i8)>
{
    s.lines()
     .zip(0 ..)
     .flat_map(|(l, y)| l.bytes()
                         .zip(0 ..)
                         .filter_map(move |(b, x)| (b == b'#').then(|| (x, y))))
}

fn neighbours(x : i8, y : i8) -> impl Iterator<Item = (i8, i8)>
{
    vec![(x-1, y-1), (x, y-1), (x+1, y-1),
         (x-1, y  ),           (x+1, y  ),
         (x-1, y+1), (x, y+1), (x+1, y+1)].into_iter()
}
