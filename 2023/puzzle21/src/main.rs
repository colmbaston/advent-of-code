use std::collections::HashSet;
use aoc::direction::Direction;

fn main()
{
    let (start, grid, width, height) = parse_grid(include_str!("../input.txt"));

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
            sequence.push(current.len() as u64);
            if sequence.len() == 3 { break }
        }

        buffer.extend(current.drain());
        for pos in buffer.drain(..).flat_map(|pos| Direction::ELEMS.into_iter().map(move |dir| dir.step(pos)))
        {
            if grid[pos.1.rem_euclid(height) as usize][pos.0.rem_euclid(width) as usize]
            {
                current.insert(pos);
            }
        }
    }

    let x         = 26_501_365 / width as u64;
    let [a, b, c] = sequence.last_chunk().unwrap();
    println!("{}", a+x*(b-a+(x-1)*((a+c)/2-b)))
}

type Pos = (i32, i32);

fn parse_grid(s : &str) -> (Pos, Vec<Vec<bool>>, i32, i32)
{
    let mut start  = None;
    let mut grid   = Vec::new();
    let mut width  = 0;
    let mut height = 0;

    for (l, y) in s.lines().zip(0..)
    {
        let mut row = Vec::new();
        for (b, x) in l.bytes().zip(0..)
        {
            row.push(b != b'#');
            if b == b'S' { start = Some((x, y)) }
            width = width.max(x+1);
        }
        grid.push(row);
        height = height.max(y+1)
    }

    (start.unwrap(), grid, width, height)
}
