use std::collections::HashSet;
use aoc::direction::Direction;

fn main()
{
    let (start, walls, width, height) = parse_grid(include_str!("../input.txt"));

    let mut current  = std::iter::once(start).collect::<HashSet<Pos>>();
    let mut buffer   = Vec::new();
    let mut sequence = Vec::new();

    for step in 0 ..
    {
        if step == 64
        {
            println!("{}", current.len())
        }

        if step % width == start.0
        {
            sequence.push((step, current.len()));
            if sequence.len() >= 3 { break }
        }

        buffer.extend(current.drain());
        for pos in buffer.drain(..).flat_map(|pos| Direction::ELEMS.into_iter().map(move |dir| dir.step(pos)))
        {
            if !walls.contains(&(pos.0.rem_euclid(width), pos.1.rem_euclid(height)))
            {
                current.insert(pos);
            }
        }
    }

    for (x, y) in sequence.into_iter()
    {
        eprintln!("f({x}) = {y}")
    }

    // hard-coded for my input for now
    let quadratic = |x : u64| (15_350*x*x + 30_415*x - 160_240) / 17_161;
    println!("{}", quadratic(26_501_365));
}

type Pos = (i32, i32);

fn parse_grid(s : &str) -> (Pos, HashSet<Pos>, i32, i32)
{
    let mut start  = None;
    let mut walls  = HashSet::new();
    let mut width  = 0;
    let mut height = 0;

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

    (start.unwrap(), walls, width, height)
}
