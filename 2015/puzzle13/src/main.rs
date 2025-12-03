#![feature(array_windows)]

use std::collections::{ HashMap, BTreeSet };
use aoc::permutations::Permutations;

const ME : &str = "Colm";

fn main()
{
    let mut table  = include_str!("../input.txt").lines().map(parse_row).collect::<HashMap<(&str, &str), i32>>();
    let mut people = table.keys().map(|&(a, _)| a).collect::<BTreeSet<&str>>();

    println!("{}", Permutations::from_sorted(people.iter().cloned()).filter_map(|p| le_reverse(&p).then(|| happiness(p, &table))).max().unwrap());

    for p in people.iter()
    {
        table.insert((ME, p), 0);
        table.insert((p, ME), 0);
    }
    people.insert(ME);

    println!("{}", Permutations::from_sorted(people.into_iter()).filter_map(|p| le_reverse(&p).then(|| happiness(p, &table))).max().unwrap());
}

fn parse_row(s : &str) -> ((&str, &str), i32)
{
    match s.split(' ').collect::<Vec<&str>>()[..]
    {
        [a, "would", b, c, "happiness", "units", "by", "sitting", "next", "to", d] =>
            ((a, &d[.. d.len()-1]), { let x = c.parse().unwrap(); if b == "gain" { x } else { -x }}),
        _ => unreachable!()
    }
}

fn le_reverse<T : Ord>(v : &[T]) -> bool
{
    v.iter().le(v.iter().rev())
}

fn happiness(mut p : Vec<&str>, table : &HashMap<(&str, &str), i32>) -> i32
{
    p.push(p[0]);
    p.array_windows().map(|&[a, b]| table.get(&(a, b)).unwrap() + table.get(&(b, a)).unwrap()).sum()
}
