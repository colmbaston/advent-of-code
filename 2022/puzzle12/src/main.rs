fn main()
{
    let mut start  = None;
    let mut end    = None;
    let mut lowest = Vec::new();
    let hill       = include_str!("../input.txt").lines().enumerate().map(|(y, row)|
    {
        row.as_bytes().iter().enumerate().map(|(x, &b)| match b
        {
            b'S' => { start = Some((x, y)); b'a' }
            b'E' => { end   = Some((x, y)); b'z' }
            b'a' => { lowest.push((x, y));  b'a' }
            b    => b
        })
        .collect::<Vec<u8>>()
    })
    .collect::<Vec<Vec<u8>>>();

    if let Some((start, end)) = start.and_then(|s| end.map(|e| (s, e)))
    {
        type Pos = (usize, usize);

        let target   = |&pos    : &Pos| pos == end;
        let height   = |&(x, y) : &Pos| hill.get(y).and_then(|row| row.get(x).copied());
        let adjacent = |&(x, y) : &Pos|
        {
            [(x+1, y), (x, y+1)].into_iter()
                                .chain(x.checked_sub(1).map(|x| (x, y)).into_iter())
                                .chain(y.checked_sub(1).map(|y| (x, y)).into_iter())
                                .filter_map(move |next| height(&(x, y)).and_then(|hc| height(&next).filter(|&hn| hn as i8 - hc as i8 <= 1)
                                                                                                   .map(|_| next)))
        };

        let mut result : Option<u32>;
        result = aoc::pathfinding::bfs(std::iter::once(start), target, adjacent);
        if let Some(steps) = result { println!("{steps}") }
        result = aoc::pathfinding::bfs(lowest.into_iter(),     target, adjacent);
        if let Some(steps) = result { println!("{steps}") }
    }
}
