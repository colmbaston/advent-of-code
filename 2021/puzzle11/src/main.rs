fn main()
{
    let mut input = include_str!("../input.txt").lines()
                                                .map(|s| s.bytes().map(|b| Some(b - b'0')).collect())
                                                .collect();

    let mut count = 0;
    for _ in 1 ..= 100
    {
        for x in 0 .. 10
        {
            for y in 0 .. 10
            {
                increment(x, y, &mut input);
            }
        }
        count += reset(&mut input);
    }
    println!("{}", count);

    for i in 101 ..
    {
        for x in 0 .. 10
        {
            for y in 0 .. 10
            {
                increment(x, y, &mut input);
            }
        }

        if reset(&mut input) == 100
        {
            println!("{}", i);
            break
        }
    }
}

fn increment(x : i8, y : i8, grid : &mut Vec<Vec<Option<u8>>>)
{
    if let Some(e) = grid.get_mut(x as usize).and_then(|v| v.get_mut(y as usize))
    {
        if let Some(f) = e
        {
            *f += 1;
            if *f > 9
            {
                *e = None;
                adjacent(x, y).for_each(|(x, y)| increment(x, y, grid));
            }
        }
    }
}

fn reset(grid : &mut [Vec<Option<u8>>]) -> usize
{
    let mut count = 0;
    for v in grid.iter_mut()
    {
        for e in v.iter_mut()
        {
            *e = e.or_else(|| { count += 1; Some(0) });
        }
    }
    count
}

fn adjacent(x : i8, y : i8) -> impl Iterator<Item = (i8, i8)>
{
    vec![(x-1, y-1), (x-1, y), (x-1, y+1), (x, y-1), (x, y+1), (x+1, y-1), (x+1, y), (x+1, y+1)].into_iter()
}
