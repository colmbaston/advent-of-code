use std::collections::HashMap;

fn main()
{
    let (direct, inverse) = parse_orbits(include_str!("../input.txt"));
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

fn parse_orbits(s : &str) -> (HashMap<&str, Vec<&str>>, HashMap<&str, &str>)
{
    let mut direct  = HashMap::new();
    let mut inverse = HashMap::new();

    for l in s.lines()
    {
        let a = &l[0..3];
        let b = &l[4..7];

        direct.entry(a).or_insert_with(Vec::new).push(b);
        inverse.insert(b, a);
    }

    (direct, inverse)
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
