fn main()
{
    let mut start  = None;
    let mut end    = None;
    let mut lowest = Vec::new();
    let hill       = include_str!("../input.txt").lines().enumerate().map(|(y, row)|
    {
        let row = row.as_bytes();
        row.iter().enumerate().for_each(|(x, b)| match b
        {
            b'S' => start = Some((x, y)),
            b'E' => end   = Some((x, y)),
            b'a' => lowest.push((x, y)),
            _    => ()
        });
        row
    })
    .collect::<Vec<&[u8]>>();

    if let Some((start, end)) = start.and_then(|s| end.map(|e| (s, e)))
    {
        type Pos = (usize, usize);

        let height = |&(x, y) : &Pos| hill.get(y).and_then(|row| row.get(x).map(|&b| match b
        {
            b'S' => b'a',
            b'E' => b'z',
            b    => b
        }));

        let target   = |&pos        : &Pos| pos == end;
        let adjacent = |&pos@(x, y) : &Pos|
        {
            [(x+1, y), (x, y+1)].into_iter()
                                .chain(x.checked_sub(1).map(|x| (x, y)).into_iter())
                                .chain(y.checked_sub(1).map(|y| (x, y)).into_iter())
                                .filter_map(move |next| height(&pos).and_then(|hc| height(&next).filter(|&hn| hn <= hc || hn-1 == hc)
                                                                                                .map(|_| next)))
        };

        let mut result : Option<u32>;
        result = aoc::pathfinding::bfs(std::iter::once(start), target, adjacent);
        if let Some(steps) = result { println!("{steps}") }
        result = aoc::pathfinding::bfs(lowest.into_iter(),     target, adjacent);
        if let Some(steps) = result { println!("{steps}") }
    }
}
