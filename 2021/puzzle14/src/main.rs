use std::collections::HashMap;

fn main()
{
    let (first, last, mut polymer, rules) = parse_polymer(include_str!("../input.txt"));

    for i in [10, 30].into_iter()
    {
        for _ in 0 .. i
        {
            polymer = react(&polymer, &rules);
        }

        let mut hist = HashMap::new();
        for (&(a, b), &freq) in polymer.iter().chain(std::iter::once((&(first, last), &1)))
        {
            *hist.entry(a).or_insert(0) += freq;
            *hist.entry(b).or_insert(0) += freq;
        }

        if let Some((min, max)) = aoc::bounds::bounds_1d(hist.values())
        {
            println!("{}", (max - min) / 2);
        }
    }
}

type Polymer = HashMap<(u8, u8), u64>;
type Rules   = HashMap<(u8, u8), u8>;

fn parse_polymer(s : &str) -> (u8, u8, Polymer, Rules)
{
    let mut i = s.split("\n\n");

    let polymer_raw = i.next().unwrap().as_bytes();
    let first       = *polymer_raw.first().unwrap();
    let last        = *polymer_raw.last().unwrap();

    let mut polymer = HashMap::new();
    for w in polymer_raw.windows(2)
    {
        *polymer.entry((w[0], w[1])).or_insert(0) += 1;
    }

    let rules = i.next().unwrap().lines().map(|t| match *t.as_bytes()
    {
        [a, b, b' ', b'-', b'>', b' ', c] => ((a, b), c),
        _                                 => unreachable!()
    })
    .collect();

    (first, last, polymer, rules)
}

fn react(polymer : &Polymer, rules : &Rules) -> Polymer
{
    let mut next = polymer.clone();

    for (&(a, b), &c) in rules.iter()
    {
        if let Some(freq) = next.get_mut(&(a, b))
        {
            let k = *polymer.get(&(a, b)).unwrap_or(&0);

            *freq                            -= k;
            *next.entry((a, c)).or_insert(0) += k;
            *next.entry((c, b)).or_insert(0) += k;
        }
    }

    next
}
