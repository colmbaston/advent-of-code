use std::collections::HashMap;

fn main()
{
    let (patterns, designs) = parse_towels(include_str!("../input.txt"));

    let mut count = 0;
    let mut sum   = 0;
    let mut cache = HashMap::new();
    for d in designs.into_iter()
    {
        let matches = match_towels(&patterns, d, &mut cache);
        count += (matches != 0) as u32;
        sum   += matches;
    }
    println!("{count}");
    println!("{sum}");
}

fn parse_towels(s : &str) -> (Vec<&str>, Vec<&str>)
{
    let (a, b) = s.split_once("\n\n").unwrap();
    (a.split(", ").collect(), b.lines().collect())
}

fn match_towels<'a>(patterns : &[&str], design : &'a str, cache : &mut HashMap<&'a str, u64>) -> u64
{
    if design.is_empty()                { return 1 }
    if let Some(&k) = cache.get(design) { return k }

    let matches = patterns.iter()
                          .filter_map(|p| design.strip_prefix(p))
                          .map(|d| match_towels(patterns, d, cache))
                          .sum();

    cache.insert(design, matches);
    matches
}
