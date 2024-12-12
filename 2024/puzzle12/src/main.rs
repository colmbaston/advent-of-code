use std::collections::{ HashMap, HashSet };
use aoc::direction::Direction;

fn main()
{
    let mut plots = HashMap::new();
    for (l, x) in include_str!("../input.txt").lines().zip(0 ..)
    {
        for (b, y) in l.bytes().zip(0 ..)
        {
            plots.insert((x, y), b);
        }
    }

    let mut regions = Vec::new();
    while let Some(&pos) = plots.keys().next()
    {
        let mut region = HashSet::new();
        purge_region(pos, &mut plots, &mut region);
        regions.push(region);
    }
    drop(plots);

    println!("{}", regions.iter().map(fence_cost_one).sum::<usize>());
    println!("{}", regions.iter().map(fence_cost_two).sum::<usize>());
}

type Pos = (i32, i32);

fn purge_region(pos : Pos, plots : &mut HashMap<Pos, u8>, region : &mut HashSet<Pos>)
{
    let plot = plots.remove(&pos).unwrap();
    region.insert(pos);

    for dir in Direction::ELEMS
    {
        let next = dir.step(pos);
        if plots.get(&next) == Some(&plot) { purge_region(next, plots, region) }
    }
}

fn fence_cost_one(region : &HashSet<Pos>) -> usize
{
    let perimeter = region.iter()
                          .map(|&pos| Direction::ELEMS.into_iter()
                                                      .filter(|dir| !region.contains(&dir.step(pos)))
                                                      .count())
                          .sum::<usize>();

    region.len() * perimeter
}

fn fence_cost_two(region : &HashSet<Pos>) -> usize
{
    let mut north_facing = region.iter().copied()
                                 .filter(|&pos| !region.contains(&Direction::North.step(pos)))
                                 .collect::<HashSet<Pos>>();

    let mut sides = 0;
    while let Some(mut current) = north_facing.iter().next().copied()
    {
        let mut start  = None;
        let mut facing = Direction::North;
        loop
        {
            let mut next = facing.clockwise().step(current);
            while region.contains(&next) && !region.contains(&facing.step(next))
            {
                if facing == Direction::North { north_facing.remove(&current); }
                current = next;
                next    = facing.clockwise().step(current);
            }
            if facing == Direction::North { north_facing.remove(&current); }

            match start
            {
                None      => start = Some(current),
                Some(pos) => if current == pos && facing == Direction::North { break }
            }
            sides += 1;

            if region.contains(&next)
            {
                current = facing.step(next);
                facing  = facing.anticlockwise();
            }
            else
            {
                facing = facing.clockwise();
            }
        }
    }

    region.len() * sides
}
