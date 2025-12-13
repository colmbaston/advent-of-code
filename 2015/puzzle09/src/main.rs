use std::collections::{ HashMap, BTreeSet };
use aoc::permutations::Permutations;

fn main()
{
    let graph  = include_str!("../input.txt").lines().map(parse).collect::<HashMap<(&str, &str), u32>>();
    let cities = graph.keys().flat_map(|&(a, b)| [a, b]).collect::<BTreeSet<&str>>();

    let (min, max) = Permutations::from_sorted(cities.into_iter()).filter(|v| le_reverse(v)).fold((u32::MAX, u32::MIN), |(a, b), p|
    {
        let d = distance(&p, &graph);
        (a.min(d), b.max(d))
    });

    println!("{}", min);
    println!("{}", max);
}

fn parse(s : &str) -> ((&str, &str), u32)
{
    match s.split(' ').collect::<Vec<&str>>()[..]
    {
        [a, "to", b, "=", c] => ((a.min(b), a.max(b)), c.parse().unwrap()),
        _                    => unreachable!()
    }
}

fn le_reverse<T : Ord>(v : &[T]) -> bool
{
    v.iter().le(v.iter().rev())
}

fn distance(path : &[&str], graph : &HashMap<(&str, &str), u32>) -> u32
{
    path.array_windows().map(|&[a, b]| graph.get(&(a.min(b), a.max(b))).unwrap()).sum()
}
