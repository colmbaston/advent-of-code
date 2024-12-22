use std::collections::HashMap;

fn main()
{
    let mut sum     = 0;
    let mut max     = 0;
    let mut windows = HashMap::new();
    for (mut state, seller) in include_str!("../input.txt").lines().map(|l| l.parse::<u64>().unwrap()).zip(1 ..)
    {
        let mut window = 0;
        let mut prev   = (state % 10) as u32;
        for i in 1 ..= 2000
        {
            const MASK : u64 = (1 << 24) - 1;
            state = ((state <<  6) ^ state) & MASK;
            state = ((state >>  5) ^ state) & MASK;
            state = ((state << 11) ^ state) & MASK;

            let price = (state % 10) as u32;
            window    = window << 8 | (9+price-prev);
            prev      = price;

            if i >= 4
            {
                let (a, b) = windows.entry(window).or_insert((0, 0));
                if std::mem::replace(a, seller) != seller
                {
                    *b += price;
                    max = max.max(*b);
                }
            }
        }
        sum += state;
    }
    println!("{sum}");
    println!("{max}");
}
