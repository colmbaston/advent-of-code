use aoc::direction::Direction;
use std::collections::HashSet;

fn main()
{
    let (bounds, guard, mut obstacles) = parse_grid(include_str!("../input.txt"));

    let mut visited = HashSet::new();
    simulate(bounds, guard, &obstacles, &mut visited);
    let positions = visited.drain().map(|(pos, _)| pos).collect::<HashSet<Pos>>();
    println!("{}", positions.len());

    let mut count = 0;
    for pos in positions
    {
        if pos == guard { continue }

        visited.clear();
        obstacles.insert(pos);
        count += simulate(bounds, guard, &obstacles, &mut visited) as u32;
        obstacles.remove(&pos);
    }
    println!("{count}")
}

type Pos = (i32, i32);

fn parse_grid(s : &str) -> (Pos, Pos, HashSet<Pos>)
{
    let mut width     = 0;
    let mut height    = 0;
    let mut guard     = (0, 0);
    let mut obstacles = HashSet::new();

    for (l, y) in s.lines().zip(0 ..)
    {
        for (b, x) in l.bytes().zip(0 ..)
        {
            match b
            {
                b'#' => { obstacles.insert((x, y)); },
                b'^' => { guard = (x, y) },
                _    => ()
            }
            width = width.max(x+1);
        }
        height = height.max(y+1);
    }

    ((width, height), guard, obstacles)
}

fn simulate((width, height) : Pos, mut guard : Pos, obstacles : &HashSet<Pos>, visited : &mut HashSet<(Pos, Direction)>) -> bool
{
    let mut dir = Direction::North;
    while (0 .. width).contains(&guard.0) && (0 .. height).contains(&guard.1)
    {
        if !visited.insert((guard, dir)) { return true }
        let next = dir.step(guard);
        if obstacles.contains(&next) { dir = dir.clockwise() } else { guard = next }
    }
    false
}
