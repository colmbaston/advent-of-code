use std::collections::HashSet;
use aoc::direction::Direction;

fn main()
{
    let (start, width, height, walls) = parse_grid(include_str!("../input.txt"));

    let mut buffer  = Vec::new();
    let mut current = HashSet::new();
    current.insert(start);

    for _ in 0 .. 64
    {
        buffer.extend(current.drain());
        for (dir, pos) in buffer.drain(..).flat_map(|pos| Direction::ELEMS.into_iter().map(move |dir| (dir, pos)))
        {
            let pos = dir.step(pos);
            if (0 .. width).contains(&pos.0) && (0 .. height).contains(&pos.1) && !walls.contains(&pos)
            {
                current.insert(pos);
            }
        }
    }
    println!("{}", current.len());
}

type Pos = (i32, i32);

fn parse_grid(s : &str) -> (Pos, i32, i32, HashSet<Pos>)
{
    let mut start  = None;
    let mut width  = 0;
    let mut height = 0;
    let mut walls  = HashSet::new();
    for (l, y) in s.lines().zip(0..)
    {
        for (b, x) in l.bytes().zip(0..)
        {
            match b
            {
                b'S' => start = Some((x, y)),
                b'.' => (),
                b'#' => { walls.insert((x, y)); },
                _    => unreachable!()
            }
            width = width.max(x+1);
        }
        height = height.max(y+1);
    }
    (start.unwrap(), width, height, walls)
}
