use std::ops::RangeInclusive;
use std::collections::HashSet;

fn main()
{
    let ranges     = parse_ranges(include_str!("../input.txt").trim_end());
    let max_digits = ranges.iter().map(|r| *r.end()).max().unwrap().ilog10()+1;

    let mut sum_one = 0;
    let mut invalid = HashSet::new();
    for digits in 1 ..= max_digits/2
    {
        for repeat in 10_u64.pow(digits-1) .. 10_u64.pow(digits)
        {
            let mut m = repeat;
            for i in 2 ..= max_digits / digits
            {
                m *= 10_u64.pow(digits);
                m += repeat;

                if ranges.iter().any(|r| r.contains(&m))
                {
                    if i == 2 { sum_one += m }
                    invalid.insert(m);
                }
            }
        }
    }
    println!("{sum_one}");
    println!("{}", invalid.into_iter().sum::<u64>());
}

fn parse_ranges(s : &str) -> Vec<RangeInclusive<u64>>
{
    s.split(',')
     .map(|r| { let (l,u) = r.split_once('-').unwrap();
                l.parse().unwrap() ..= u.parse().unwrap() })
     .collect()
}
