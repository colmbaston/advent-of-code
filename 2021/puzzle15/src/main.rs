use std::cmp::Reverse;
use std::collections::{ BinaryHeap, HashSet };

fn main()
{
    let input = include_str!("../input.txt").lines().map(|l| l.bytes().map(|b| b - b'0').collect::<Vec<_>>()).collect::<Vec<_>>();
    let lx    = input[0].len() as i32;
    let ly    = input.len()    as i32;

    println!("{}", dijkstra(lx - 1, ly - 1, |x, y|
    {
        input.get(y as usize).and_then(|l| l.get(x as usize).cloned())
    }));

    println!("{}", dijkstra(5*lx - 1, 5*ly - 1, |x, y|
    {
        ((0 .. 5*lx).contains(&x) && (0 .. 5*ly).contains(&y)).then(||
        {
            (input[(y % ly) as usize][(x % lx) as usize] + ((x / lx) + (y / ly)) as u8 - 1) % 9 + 1
        })
    }));
}

fn dijkstra(tx : i32, ty : i32, cave : impl Fn(i32, i32) -> Option<u8>) -> u32
{
    let mut queue   = BinaryHeap::new();
    let mut visited = HashSet::new();
    queue.push((Reverse(0), 0, 0));

    while let Some((Reverse(r), x, y)) = queue.pop()
    {
        if x == tx && y == ty      { return r }
        if !visited.insert((x, y)) { continue }

        queue.extend(adjacent(x, y).filter_map(|(x, y)| cave(x, y).map(|s| (Reverse(r + s as u32), x, y))));
    }

    unreachable!()
}

fn adjacent(x : i32, y : i32) -> impl Iterator<Item = (i32, i32)>
{
    vec![(x-1, y), (x+1, y), (x, y-1), (x, y+1)].into_iter()
}
