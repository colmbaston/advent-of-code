use std::collections::HashSet;
use aoc::direction::Direction;

fn main()
{
    let input = include_str!("../input.txt").trim_end().split(", ").map(parse_inst).collect::<Vec<_>>();

    let mut pos = (0i32, 0i32);
    let mut dir = Direction::North;

    let mut visited  = HashSet::new();
    let mut location = None;

    for &(clockwise, k) in input.iter()
    {
        dir = if clockwise { dir.clockwise() } else { dir.anticlockwise() };

        for _ in 0 .. k
        {
            if !visited.insert(pos) { location.get_or_insert(pos); }
            pos = dir.step(pos);
        }
    }
    println!("{}", pos.0.abs() + pos.1.abs());

    if let Some((x, y)) = location
    {
        println!("{}", x.abs() + y.abs());
    }
}

fn parse_inst(s : &str) -> (bool, i32)
{
    let (a, b) = s.split_at(1);
    (a == "R", b.parse().unwrap())
}
