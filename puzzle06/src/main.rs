use parsing::*;
use std::collections::HashMap;

fn main()
{
    let (direct, inverse) = parse_orbits(include_str!("../input.txt")).unwrap().1;
    println!("{}", count_orbits(&direct, "COM", 0));

    let path_from_you : Vec<_> = path_to_root(&inverse, "YOU").collect();
    let path_from_san : Vec<_> = path_to_root(&inverse, "SAN").collect();
    for (i, (x, y)) in path_from_you.iter().rev().zip(path_from_san.iter().rev()).enumerate()
    {
        if x != y
        {
            println!("{}", path_from_you.len() + path_from_san.len() - 2 * i);
            break
        }
    }
}

fn parse_orbits(s : &str) -> IResult<&str, (HashMap<&str, Vec<&str>>, HashMap<&str, &str>)>
{
    fold_many0(tuple((alphanumeric1, char(')'), alphanumeric1, newline)),
               (HashMap::new(), HashMap::new()),
               |(mut direct, mut inverse) : (HashMap<_, Vec<_>>, _), (a, _, b, _)|
               {
                   direct.entry(a).and_modify(|v| v.push(b)).or_insert_with(|| vec![b]);
                   inverse.insert(b, a);
                   (direct, inverse)
               })
               (s)
}

fn count_orbits(orbits : &HashMap<&str, Vec<&str>>, body : &str, depth : u64) -> u64
{
    match orbits.get(body)
    {
        None    => depth,
        Some(v) => v.iter().fold(depth, |a, x| a + count_orbits(orbits, x, depth + 1))
    }
}

fn path_to_root<'a>(orbits : &'a HashMap<&str, &str>, source : &'a str) -> impl Iterator<Item = &'a str>
{
    let mut current = source;
    std::iter::from_fn(move || { current = orbits.get(current)?; Some(current) })
}
