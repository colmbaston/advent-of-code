use std::collections::{ VecDeque, HashSet, HashMap, hash_map::Entry };
use aoc::direction::Direction;

fn main()
{
    let (start, _, maze) = parse_maze(include_str!("../input.txt"));

    let mut visited = HashMap::<Pos, i32>::new();
    let mut queue   = VecDeque::new();
    queue.push_back((0, start));
    while let Some((steps, pos)) = queue.pop_front()
    {
        match visited.entry(pos)
        {
            Entry::Occupied(_) => continue,
            Entry::Vacant(e)   => { e.insert(steps); }
        }

        for dir in Direction::ELEMS.into_iter()
        {
            let next = dir.step(pos);
            if maze.contains(&next) { queue.push_back((steps+1, next)) }
        }
    }
    drop(queue);

    let mut one = 0;
    let mut two = 0;
    for (i, &p1) in maze.iter().enumerate()
    {
        for &p2 in maze.iter().skip(i+1)
        {
            let manhattan = p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1);
            if manhattan > 20 { continue }

            if visited[&p1].abs_diff(visited[&p2]).checked_sub(manhattan).is_some_and(|saved| saved >= 100)
            {
                one += (manhattan <= 2) as u32;
                two += 1;
            }
        }
    }
    println!("{one}");
    println!("{two}");
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
                _    => ()
            }
        }
    }
    (start, end, maze)
}
