use aoc::direction::Direction;
use std::collections::HashSet;

fn main()
{
    let (map, heads) = parse_map(include_str!("../input.txt"));
    println!("{}", heads.iter().map(|&pos| dfs(pos, &map, &mut Some(HashSet::new()))).sum::<u32>());
    println!("{}", heads.iter().map(|&pos| dfs(pos, &map, &mut None)).sum::<u32>());
}

type Pos = (usize, usize);

fn parse_map(s : &str) -> (Vec<Vec<u8>>, Vec<Pos>)
{
    let mut map   = Vec::new();
    let mut heads = Vec::new();
    for (y, l) in s.lines().enumerate()
    {
        let mut row = Vec::new();
        for (x, b) in l.bytes().enumerate()
        {
            row.push(b);
            if b == b'0' { heads.push((x, y)) }
        }
        map.push(row)
    }
    (map, heads)
}

fn dfs(pos : Pos, map : &[Vec<u8>], visited : &mut Option<HashSet<Pos>>) -> u32
{
    let height = map[pos.1][pos.0];
    if height == b'9' { return visited.as_mut().map_or(1, |s| s.insert(pos) as u32) }

    Direction::ELEMS.into_iter().filter_map(|dir|
    {
        let (x, y) = dir.checked_step(pos)?;
        let h      = *map.get(y)?.get(x)?;
        (h == height+1).then(|| dfs((x, y), map, visited))
    })
    .sum::<u32>()
}
