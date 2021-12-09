use std::collections::{ HashMap, BinaryHeap };

fn main()
{
    let mut input = include_str!("../input.txt").lines()
                                                .zip(0 ..)
                                                .flat_map(|(l, y)| l.bytes().zip(0 ..).map(move |(b, x)| ((x, y), b - b'0')))
                                                .collect::<HashMap<(i32, i32), u8>>();

    let mut low_points = Vec::new();
    for (&k, &h) in input.iter()
    {
        if adjacents(k).filter_map(|a| input.get(&a)).all(|&g| h < g)
        {
            low_points.push(h);
        }
    }
    println!("{}", low_points.len() + low_points.into_iter().map(|k| k as usize).sum::<usize>());

    let mut basins = BinaryHeap::new();
    while let Some(&k) = input.keys().next()
    {
        let size = nuke_basin(k, &mut input);
        if size > 0 { basins.push(size) }
    }
    println!("{}", basins.into_iter().take(3).product::<usize>());
}

fn adjacents((x, y) : (i32, i32)) -> impl Iterator<Item = (i32, i32)>
{
    vec![(x-1, y), (x+1, y), (x, y-1), (x, y+1)].into_iter()
}

fn nuke_basin(k : (i32, i32), cave : &mut HashMap<(i32, i32), u8>) -> usize
{
    match cave.remove(&k)
    {
        None | Some(9) => 0,
        Some(_)        => 1 + adjacents(k).map(|a| nuke_basin(a, cave)).sum::<usize>()
    }
}
