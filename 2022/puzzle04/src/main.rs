use std::ops::RangeInclusive;

fn main()
{
    let (one, two) = include_str!("../input.txt").lines()
                                                 .fold((0, 0), |(one, two), l|
                                                 {
                                                     let (mut small, mut large) = parse_ranges(l);
                                                     if small.len() > large.len() { std::mem::swap(&mut small, &mut large) }

                                                     let contains_start = large.contains(small.start());
                                                     let contains_end   = large.contains(small.end());

                                                     (one + (contains_start && contains_end) as u32,
                                                      two + (contains_start || contains_end) as u32)
                                                 });

    println!("{one}");
    println!("{two}");
}

fn parse_ranges(s : &str) -> (RangeInclusive<u8>, RangeInclusive<u8>)
{
    let mut nums = s.split(|c : char| !c.is_ascii_digit()).filter_map(|n| n.parse().ok());
    let first    = nums.next().unwrap() ..= nums.next().unwrap();
    let second   = nums.next().unwrap() ..= nums.next().unwrap();

    (first, second)
}
