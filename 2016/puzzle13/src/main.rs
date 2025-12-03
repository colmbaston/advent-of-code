use aoc::direction::Direction;
use std::collections::{ VecDeque, HashSet };

fn main()
{
    let input = include_str!("../input.txt").trim_end().parse::<u32>().unwrap();

    const TARGET : Pos = (31, 39);
    let mut distinct   = 0;
    let mut visited    = HashSet::new();
    let mut queue      = VecDeque::new();
    queue.push_back(((1, 1), 0));

    while let Some((pos, steps)) = queue.pop_front()
    {
        if !visited.insert(pos) { continue }
        if steps <= 50          { distinct += 1 }
        if pos == TARGET        { println!("{steps}"); break }

        queue.extend(Direction::ELEMS.into_iter()
                                     .filter_map(|dir| dir.checked_step(pos))
                                     .filter(|&pos| !visited.contains(&pos) && open(pos, input))
                                     .map(|pos| (pos, steps+1)));
    }
    println!("{distinct}");
}

type Pos = (u32, u32);

fn open((x, y) : Pos, k : u32) -> bool
{
    (x*x + 3*x + 2*x*y + y + y*y + k).count_ones().is_multiple_of(2)
}
