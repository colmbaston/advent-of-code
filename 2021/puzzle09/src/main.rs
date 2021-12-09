use std::collections::HashMap;

fn main()
{
    let input = include_str!("../input.txt").lines()
                                            .map(|l| l.bytes().map(|b| b - b'0').collect::<Vec<u8>>())
                                            .collect::<Vec<Vec<u8>>>();

    let mut low_points = Vec::new();
    for i in 0 .. input.len()
    {
        for j in 0 .. input[0].len()
        {
            let h = input[i][j];
            if adjacents((i, j)).filter_map(|(x, y)| input.get(x).and_then(|v| v.get(y))).all(|&g| h < g)
            {
                low_points.push(h);
            }
        }
    }
    println!("{}", low_points.iter().map(|&k| k as usize).sum::<usize>() + low_points.len());

    let mut input = input.into_iter()
                         .enumerate()
                         .flat_map(|(y, v)| v.into_iter().enumerate().map(move |(x, b)| ((x, y), b)))
                         .collect::<HashMap<(usize, usize), u8>>();

    let mut basins = Vec::new();
    while let Some(&k) = input.keys().next()
    {
        let size = nuke_basin(k, &mut input);
        if size > 0 { basins.push(size) }
    }
    basins.sort_unstable();
    println!("{}", basins.into_iter().rev().take(3).product::<usize>());
}

fn adjacents((x, y) : (usize, usize)) -> impl Iterator<Item = (usize, usize)>
{
    vec![(x.wrapping_sub(1), y), (x+1, y), (x, y.wrapping_sub(1)), (x, y+1)].into_iter()
}

fn nuke_basin(k : (usize, usize), cave : &mut HashMap<(usize, usize), u8>) -> usize
{
    match cave.remove(&k)
    {
        None | Some(9) => 0,
        Some(_)        => 1 + adjacents(k).map(|a| nuke_basin(a, cave)).sum::<usize>()
    }
}
