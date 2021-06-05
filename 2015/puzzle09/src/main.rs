use std::collections::{ HashMap, BTreeSet };

mod permutations;
use permutations::Permutations;

fn main()
{
    let graph  = include_str!("../input.txt").lines().map(parse).collect::<HashMap<(&str, &str), u32>>();
    let cities = graph.keys().flat_map(|&(a, b)| vec![a, b].into_iter()).collect::<BTreeSet<&str>>();

    let (min, max) = Permutations::new(cities.into_iter()).fold((u32::MAX, u32::MIN), |(a, b), p|
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

fn distance(path : &[&str], graph : &HashMap<(&str, &str), u32>) -> u32
{
    path.windows(2).map(|w| graph.get(&(w[0].min(w[1]), w[0].max(w[1]))).unwrap()).sum()
}
