use std::collections::HashMap;

fn main()
{
    let mut rates   = HashMap::new();
    let mut tunnels = HashMap::new();
    for (valve, rate, ts) in include_str!("../input.txt").lines().filter_map(parse_valve)
    {
        if rate > 0 { rates.insert(valve, rate); }
        tunnels.insert(valve, ts);
    }

    println!("{rates:?}");
    println!("{tunnels:?}");
}

fn parse_valve(s : &str) -> Option<(&str, u32, Vec<&str>)>
{
    let s          = s.strip_prefix("Valve ")?;
    let (valve, s) = s.split_at(s.find(' ')?);
    let s          = s.strip_prefix(" has flow rate=")?;
    let (rate,  s) = s.split_at(s.find(';')?);
    let s          = s.strip_prefix("; tunnels lead to valves ").or(
                     s.strip_prefix("; tunnel leads to valve "))?;

    Some((valve, rate.parse().ok()?, s.split(", ").collect()))
}
