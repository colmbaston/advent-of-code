use std::collections::HashMap;

fn main()
{
    let mut input = include_str!("../input.txt").lines().map(|x| x.parse::<u32>().unwrap()).collect::<Vec<_>>();

    input.push(0);
    input.sort();
    input.push(input.last().cloned().unwrap() + 3);

    let mut one   = 0;
    let mut three = 0;
    for w in input.windows(2)
    {
        match w[1] - w[0]
        {
            1 => one   += 1,
            3 => three += 1,
            _ => ()
        }
    }
    println!("{}", one * three);

    let mut cache = HashMap::with_capacity(input.len());
    cache.insert(input.last().cloned().unwrap(), 1u64);
    for i in input.into_iter().rev().skip(1)
    {
        cache.insert(i, cache.get(&(i+1)).cloned().unwrap_or(0)
                      + cache.get(&(i+2)).cloned().unwrap_or(0)
                      + cache.get(&(i+3)).cloned().unwrap_or(0));
    }
    println!("{}", cache.get(&0).unwrap());
}
