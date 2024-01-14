use std::collections::HashMap;

fn main()
{
    let mut banks = include_str!("../input.txt").split_whitespace()
                                                .map(|k| k.parse().unwrap())
                                                .collect::<Vec<u8>>();

    let mut steps   = 0;
    let mut visited = HashMap::new();
    loop
    {
        if let Some(i) = visited.insert(banks.clone(), steps)
        {
            println!("{steps}");
            println!("{}", steps - i);
            break
        }
        redistribute(&mut banks);
        steps += 1
    }
}

fn redistribute(banks : &mut [u8])
{
    let i       = banks.iter().enumerate().max_by(|(i, a), (j, b)| a.cmp(b).then(j.cmp(i))).unwrap().0;
    let amount  = std::mem::replace(&mut banks[i], 0);
    let quot    = amount / banks.len() as u8;
    let mut rem = amount % banks.len() as u8;

    let (lower, upper) = banks.split_at_mut(i+1);
    for bank in upper.iter_mut().chain(lower.iter_mut())
    {
        *bank += quot + (rem > 0) as u8;
        rem = rem.saturating_sub(1)
    }
}
