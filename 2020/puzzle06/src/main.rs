use std::collections::HashSet;

fn main()
{
    let input = include_str!("../input.txt").split("\n\n").map(|s| s.lines().collect::<Vec<_>>()).collect::<Vec<_>>();

    println!("{}", input.iter().map(|g|        union(g.iter().cloned())).sum::<usize>());
    println!("{}", input.iter().map(|g| intersection(g.iter().cloned())).sum::<usize>());
}

fn union<'a>(i : impl Iterator<Item = &'a str>) -> usize
{
    let mut s = HashSet::new();
    i.for_each(|x| x.bytes().for_each(|b| { s.insert(b); }));
    s.len()
}

fn intersection<'a>(i : impl Iterator<Item = &'a str>) -> usize
{
    let mut s = (b'a' ..= b'z').collect();
    i.for_each(|x| s = x.bytes().collect::<HashSet<_>>().intersection(&s).cloned().collect());
    s.len()
}
