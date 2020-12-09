use std::cmp::Ordering;
use std::collections::HashSet;

fn main()
{
    let input = include_str!("../input.txt").lines().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();

    let mut current = input.iter().take(25).cloned().collect::<HashSet<_>>();
    let mut invalid = None;

    for (i, j) in input.iter().cloned().zip(input.iter().skip(25).cloned())
    {
        if !current.iter().any(|&x| { let k = j - x; k != j && current.contains(&k) })
        {
            println!("{}", j);
            invalid = Some(j);
            break
        }

        current.remove(&i);
        current.insert(j);
    }

    if let Some(target) = invalid
    {
        let mut start   = 0;
        let mut end     = 0;
        let mut sum     = 0;

        loop
        {
            match sum.cmp(&target)
            {
                Ordering::Equal   => break,
                Ordering::Less    => { sum += input[end];   end   += 1 },
                Ordering::Greater => { sum -= input[start]; start += 1 }
            }
        }

        println!("{}", input[start .. end].iter().min().unwrap()
                     + input[start .. end].iter().max().unwrap());
    }
}
