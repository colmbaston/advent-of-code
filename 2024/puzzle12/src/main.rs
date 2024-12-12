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
    region.len() * region.iter()
                         .map(|&pos| Direction::ELEMS.into_iter()
                                                     .filter(|dir| !region.contains(&dir.step(pos)))
                                                     .count())
                         .sum::<usize>()
}

fn fence_cost_two(region : &HashSet<Pos>) -> usize
{
    region.len() * region.iter().map(|&pos|
    {
        Direction::ELEMS.into_iter().filter(|dir|
        {
            let a = dir.step(pos);
            let b = dir.clockwise().step(pos);
            let c = dir.step(b);

            matches!([region.contains(&a), region.contains(&b), region.contains(&c)], [false, false, _] | [true, true, false])
        })
        .count()
    })
    .sum::<usize>()
}
