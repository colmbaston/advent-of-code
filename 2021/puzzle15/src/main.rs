use std::cmp::Reverse;
use std::collections::{ BinaryHeap, HashSet };

fn main()
{
    let input = include_str!("../input.txt").lines().map(|l| l.bytes().map(|b| b - b'0').collect::<Vec<_>>()).collect::<Vec<_>>();
    let lx    = input[0].len() as i32;
    let ly    = input.len()    as i32;

    println!("{}", dijkstra(  lx - 1,   ly - 1, |x, y| input.get(y as usize).and_then(|l| l.get(x as usize).cloned())));
    println!("{}", dijkstra(5*lx - 1, 5*ly - 1, |x, y| (0 <= x && x < 5*lx && 0 <= y && y < 5*ly).then(||
                                                       (input[(y % ly) as usize][(x % lx) as usize] + (x / lx + y / ly) as u8 - 1) % 9 + 1)));
}

fn dijkstra(tx : i32, ty : i32, risk : impl Fn(i32, i32) -> Option<u8>) -> u32
{
    let mut queue   = BinaryHeap::new();
    let mut visited = HashSet::new();

    queue.push((Reverse(0), 0, 0));
    while let Some((Reverse(r), x, y)) = queue.pop()
    {
        if x == tx && y == ty      { return r }
        if !visited.insert((x, y)) { continue }

        queue.extend(aoc::search::ortho_2d(x, y).into_iter().filter_map(|(x, y)| risk(x, y).map(|s| (Reverse(r + s as u32), x, y))));
    }
    unreachable!()
}
