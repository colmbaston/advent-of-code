use std::collections::HashMap;

fn main()
{
    let input = include_str!("../input.txt").trim_end().split(',').map(|x| x.parse::<u32>().unwrap()).collect::<Vec<_>>();

    let (k, rest)   = input.split_last().unwrap();
    let mut prev    = *k;
    let mut visited = rest.iter().cloned().zip(1..).collect::<HashMap<u32, u32>>();

    for i in input.len() as u32 .. 2020 { prev = step(prev, i, &mut visited) }
    println!("{}", prev);
    for i in 2020 .. 30000000           { prev = step(prev, i, &mut visited) }
    println!("{}", prev);
}

fn step(prev : u32, i : u32, visited : &mut HashMap<u32, u32>) -> u32
{
    match visited.insert(prev, i)
    {
        None    => 0,
        Some(j) => i - j
    }
}
