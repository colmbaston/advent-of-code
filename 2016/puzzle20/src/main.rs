use std::ops::RangeInclusive;

fn main()
{
    let mut ranges = include_str!("../input.txt").lines().map(parse_range).collect::<Vec<RangeInclusive<u32>>>();
    ranges.sort_unstable_by_key(|r| *r.start());

    let mut merged  = Vec::new();
    let mut current = ranges[0].clone();
    for r in ranges.iter().skip(1)
    {
        if current.end().saturating_add(1) >= *r.start()
        {
            current = *current.start() ..= *current.end().max(r.end())
        }
        else
        {
            merged.push(std::mem::replace(&mut current, r.clone()))
        }
    }
    merged.push(current);

    // assume 0 and u32::MAX are blocked
    println!("{}", merged[0].end()+1);
    println!("{}", merged.array_windows().map(|[a, b]| b.start() - a.end() - 1).sum::<u32>());
}

fn parse_range(s : &str) -> RangeInclusive<u32>
{
    let (a, b) = s.split_once('-').unwrap();
    a.parse().unwrap() ..= b.parse().unwrap()
}
