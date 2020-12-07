use std::collections::HashMap;

fn main()
{
    let rules = include_str!("../input.txt").lines().map(parse_rule).collect::<HashMap<_, _>>();

    let mut cache = HashMap::with_capacity(rules.len());
    println!("{}", rules.keys().filter(|k| contains(k, "shiny gold", &rules, &mut cache)).count());

    let mut cache = HashMap::with_capacity(rules.len());
    println!("{}", count_bags("shiny gold", &rules, &mut cache) - 1);
}

fn parse_rule(s : &str) -> (&str, Vec<(u32, &str)>)
{
    let mut i = s.split(" bags contain ");
    let c = i.next().unwrap();
    let s = i.next().unwrap();

    (c, match s
    {
        "no other bags." => Vec::new(),
        _                => s.split(", ").map(|t|
        {
            let (n, d) = t.split_at(t.find(|c : char| !c.is_ascii_digit()).unwrap());
            (n.parse().unwrap(), d[1..].split(" bag").next().unwrap())
        })
        .collect()
    })
}

fn contains<'a>(value : &'a str, target : &'a str, rules : &HashMap<&'a str, Vec<(u32, &'a str)>>, cache : &mut HashMap<&'a str, bool>) -> bool
{
    match cache.get(&value)
    {
        Some(r) => *r,
        None    =>
        {
            let result = rules.get(&value)
                              .unwrap()
                              .iter()
                              .any(|(_, b)| *b == target || contains(b, target, rules, cache));

            cache.insert(value, result);
            result
        }
    }
}

fn count_bags<'a>(value : &'a str, rules : &HashMap<&'a str, Vec<(u32, &'a str)>>, cache : &mut HashMap<&'a str, u32>) -> u32
{
    match cache.get(&value)
    {
        Some(r) => *r,
        None    =>
        {
            let result = 1 + rules.get(&value)
                                  .unwrap()
                                  .iter()
                                  .map(|(n, b)| n * count_bags(b, rules, cache))
                                  .sum::<u32>();

            cache.insert(value, result);
            result
        }
    }
}
