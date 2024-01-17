use std::collections::HashSet;
use aoc::direction::Direction;

fn main()
{
    let input    = include_str!("../input.txt").trim_end();
    let mut grid = HashSet::new();
    for row in 0 .. 128
    {
        for (byte, i) in knot_hash::hash(format!("{input}-{row}").as_bytes()).iter().zip(0 ..)
        {
            for bit in 0 .. 8
            {
                if byte & (1 << (7 - bit)) != 0
                {
                    grid.insert((row, 8*i + bit));
                }
            }
        }
    }
    println!("{}", grid.len());

    let mut count = 0;
    while let Some(&pos) = grid.iter().next()
    {
        purge(pos, &mut grid);
        count += 1
    }
    println!("{count}");
}

type Pos = (i32, i32);

fn purge(pos : Pos, grid : &mut HashSet<Pos>)
{
    if grid.remove(&pos)
    {
        Direction::ELEMS.into_iter().for_each(|dir| purge(dir.step(pos), grid))
    }
}
