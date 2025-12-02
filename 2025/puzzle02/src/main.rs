use std::ops::RangeInclusive;

fn main()
{
    let ranges = parse_ranges(include_str!("../input.txt").trim_end());

    let mut sum_one = 0;
    let mut sum_two = 0;
    for range in ranges
    {
        for n in range
        {
            let digits = n.ilog10() + 1;

            // all primes <= digits(u32::MAX)
            'outer: for p in [2, 3, 5, 7, 11, 13, 17, 19]
            {
                // redundant, but increases speed by avoiding
                // the more expensive mod check in many cases
                if digits < p      { continue }
                if digits % p != 0 { continue }

                let divisor = 10_u64.pow(digits / p);
                let first   = n % divisor;

                let mut m = n;
                for _ in 1 .. p
                {
                    m /= divisor;
                    if first != m % divisor { continue 'outer }
                }

                if p == 2 { sum_one += n }
                sum_two += n;
                break
            }
        }
    }
    println!("{sum_one}");
    println!("{sum_two}");
}

fn parse_ranges(s : &str) -> Vec<RangeInclusive<u64>>
{
    s.split(',')
     .map(|r| { let (l,u) = r.split_once('-').unwrap();
                l.parse().unwrap() ..= u.parse().unwrap() })
     .collect()
}
