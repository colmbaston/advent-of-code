use std::collections::{ HashSet, VecDeque };
use aoc::direction::Direction;

fn main()
{
    let bytes = include_str!("../input.txt").lines().map(parse_byte).collect::<Vec<Pos>>();
    println!("{}", bfs(&bytes.iter().copied().take(1024).collect()).unwrap());

    let prefixes = (1 ..= bytes.len()).map(|i| &bytes[.. i]).collect::<Vec<&[Pos]>>();
    let (x, y)   = bytes[prefixes.partition_point(|prefix| bfs(&prefix.iter().copied().collect()).is_some())];
    println!("{x},{y}");
}

type Pos = (i32, i32);
const MAX_X : i32 = 70;
const MAX_Y : i32 = 70;

fn parse_byte(s : &str) -> Pos
{
    let (x, y) = s.split_once(',').unwrap();
    (x.parse().unwrap(), y.parse().unwrap())
}

fn bfs(obstructions : &HashSet<Pos>) -> Option<u32>
{
    let mut visited = HashSet::new();
    let mut queue   = VecDeque::new();
    queue.push_back((0, (0, 0)));

    while let Some((cost, pos)) = queue.pop_front()
    {
        if pos == (MAX_X, MAX_Y) { return Some(cost) }
        if !visited.insert(pos)  { continue          }
        queue.extend(Direction::ELEMS.into_iter()
                                     .map(|dir| (cost+1, dir.step(pos)))
                                     .filter(|(_, next)| (0 ..= MAX_X).contains(&next.0) &&
                                                         (0 ..= MAX_Y).contains(&next.1) &&
                                                         !obstructions.contains(next)))
    }
    None
}
