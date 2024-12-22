use std::collections::HashMap;

fn main()
{
    let numbers = include_str!("../input.txt").lines()
                                              .map(|l| Prng { state: l.parse().unwrap() }.take(2001).collect::<Vec<u64>>())
                                              .collect::<Vec<Vec<u64>>>();

    println!("{}", numbers.iter().map(|seq| seq[2000]).sum::<u64>());

    let mut windows = HashMap::new();
    for (seller, sequence) in numbers.into_iter().enumerate()
    {
        let prices  = sequence.into_iter().map(|k| (k % 10) as i8).collect::<Vec<i8>>();
        let changes = prices.windows(2).map(|w| w[1] - w[0]).collect::<Vec<i8>>();

        for (w, &price) in changes.windows(4).zip(prices.iter().skip(4))
        {
            windows.entry((w[0], w[1], w[2], w[3]))
                   .or_insert_with(HashMap::new)
                   .entry(seller)
                   .or_insert(price);
        }
    }
    println!("{}", windows.into_values().map(|m| m.into_values().map(|v| v as u32).sum::<u32>()).max().unwrap());
}

struct Prng { state: u64 }

impl Iterator for Prng
{
    type Item = u64;

    fn next(&mut self) -> Option<u64>
    {
        const MASK : u64 = (1 << 24) - 1;

        let result = self.state;
        self.state = ((self.state <<  6) ^ self.state) & MASK;
        self.state = ((self.state >>  5) ^ self.state) & MASK;
        self.state = ((self.state << 11) ^ self.state) & MASK;
        Some(result)
    }
}
