use std::{ cmp::Reverse, collections::{ hash_map::Entry, BinaryHeap, HashSet, HashMap }};
use aoc::direction::Direction;

fn main()
{
    let (start, end, maze) = parse_maze(include_str!("../input.txt"));

    let mut best    = None;
    let mut visited = HashMap::new();
    let mut queue   = BinaryHeap::new();
    queue.push(Reverse((0, start, Direction::East)));
    while let Some(Reverse((steps, pos, dir))) = queue.pop()
    {
        if pos == end && *best.get_or_insert(steps) < steps { break }

        match visited.entry((pos, dir))
        {
            Entry::Occupied(_) => continue,
            Entry::Vacant(e)   => { e.insert(steps); }
        }

        queue.push(Reverse((steps+1000, pos, dir.clockwise())));
        queue.push(Reverse((steps+1000, pos, dir.anticlockwise())));

        let next = dir.step(pos);
        if maze.contains(&next)
        {
            queue.push(Reverse((steps+1, next, dir)))
        }
    }
    drop(queue);
    println!("{}", best.unwrap());

    let mut paths = HashSet::new();
    let mut stack = Direction::ELEMS.into_iter().map(|dir| (best.unwrap(), end, dir)).collect::<Vec<(u32, Pos, Direction)>>();
    while let Some((steps, pos, dir)) = stack.pop()
    {
        if visited.get(&(pos, dir)) == Some(&steps)
        {
            paths.insert(pos);
            stack.push((steps.wrapping_sub(1000), pos,                      dir.clockwise()));
            stack.push((steps.wrapping_sub(1000), pos,                      dir.anticlockwise()));
            stack.push((steps.wrapping_sub(1),    dir.opposite().step(pos), dir));
        }
    }
    println!("{}", paths.len());
}

type Pos = (i32, i32);

fn parse_maze(s : &str) -> (Pos, Pos, HashSet<Pos>)
{
    let mut start = (0, 0);
    let mut end   = (0, 0);
    let mut maze  = HashSet::new();
    for (l, y) in s.lines().zip(0 ..)
    {
        for (b, x) in l.bytes().zip(0 ..)
        {
            match b
            {
                b'S' => { start = (x, y); maze.insert(start); },
                b'E' => { end   = (x, y); maze.insert(end);   },
                b'.' => { maze.insert((x, y)); },
                _    => {}
            }
        }
    }
    (start, end, maze)
}
