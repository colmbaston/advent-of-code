use std::collections::HashMap;

fn main()
{
    let mut input = include_str!("../input.txt").lines().map(|x| x.parse::<u32>().unwrap()).collect::<Vec<_>>();
    input.push(0);
    input.sort();
    input.push(input.last().cloned().unwrap() + 3);

    let mut n1 = 0;
    let mut n3 = 0;
    for (i, j) in input.iter().zip(input.iter().skip(1))
    {
        match j - i
        {
            1 => n1 += 1,
            3 => n3 += 1,
            _ => ()
        }
    }
    println!("{}", n1 * n3);

    let mut cache = HashMap::with_capacity(input.len());
    cache.insert(input.last().cloned().unwrap(), 1u64);
    for i in input.into_iter().rev().skip(1)
    {
        cache.insert(i, (1 ..= 3).map(|j| cache.get(&(i+j)).cloned().unwrap_or(0)).sum());
    }
    println!("{}", cache.get(&0).unwrap());
}
