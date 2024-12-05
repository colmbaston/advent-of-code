use std::collections::{ HashMap, HashSet };

fn main()
{
    let (a, b)  = include_str!("../input.txt").split_once("\n\n").unwrap();
    let rules   = parse_rules(a);

    let mut sum_one = 0;
    let mut sum_two = 0;
    for mut manual in b.lines().map(|l| l.split(',').map(|k| k.parse().unwrap()).collect::<Vec<u32>>())
    {
        if sort_manual(&rules, &mut manual)
        {
            sum_one += manual[manual.len()/2];
        }
        else
        {
            sum_two += manual[manual.len()/2];
        }
    }
    println!("{sum_one}");
    println!("{sum_two}");
}

fn parse_rules(s : &str) -> HashMap<u32, HashSet<u32>>
{
    let mut rules = HashMap::new();
    for l in s.lines()
    {
        let (a, b) = l.split_once('|').unwrap();
        rules.entry(a.parse().unwrap())
             .or_insert(HashSet::new())
             .insert(b.parse().unwrap());
    }
    rules
}

fn sort_manual(rules : &HashMap<u32, HashSet<u32>>, manual : &mut [u32]) -> bool
{
    let mut sorted = true;
    for i in 0 .. manual.len()
    {
        while let Some(j) = (0 .. i).find(|&j| rules.get(&manual[i]).is_some_and(|s| s.contains(&manual[j])))
        {
            sorted = false;
            manual.swap(i, j);
        }
    }
    sorted
}
