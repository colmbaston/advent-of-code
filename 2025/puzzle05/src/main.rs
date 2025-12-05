use std::ops::RangeInclusive;

fn main()
{
    let (mut fresh, available) = parse_ids(include_str!("../input.txt"));

    fresh.sort_unstable_by_key(|r| *r.start());
    let mut merged = fresh[.. 1].to_vec();
    for r in fresh[1 ..].iter()
    {
        let i = merged.partition_point(|s| s.end()+1 < *r.start());
        if i == merged.len()
        {
            merged.push(r.clone());
        }
        else
        {
            merged[i] = *merged[i].start() ..= *merged[i].end().max(r.end());
        }
    }

    println!("{}", available.into_iter()
                            .filter(|a| merged.get(merged.partition_point(|r| r.end() < a))
                                              .map(|r| r.contains(a))
                                              .unwrap_or(false))
                            .count());

    println!("{}", merged.into_iter()
                         .map(|r| 1+r.end()-r.start())
                         .sum::<u64>());
}

fn parse_ids(s : &str) -> (Vec<RangeInclusive<u64>>, Vec<u64>)
{
    let (a, b) = s.split_once("\n\n").unwrap();

    (a.lines().map(|l| l.split_once('-').map(|(l, u)| l.parse().unwrap() ..= u.parse().unwrap()).unwrap()).collect(),
     b.lines().map(|l| l.parse().unwrap()).collect())
}
