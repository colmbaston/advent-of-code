fn main()
{
    let (one, two) = include_str!("../input.txt").lines()
                                                 .fold((0, 0), |(one, two), l|
                                                 {
                                                     let mut nums  = l.split(|c : char| !c.is_ascii_digit()).filter_map(|n| n.parse::<u8>().ok());
                                                     let mut small = nums.next().unwrap() ..= nums.next().unwrap();
                                                     let mut large = nums.next().unwrap() ..= nums.next().unwrap();
                                                     if small.len() > large.len() { std::mem::swap(&mut small, &mut large) }

                                                     let contains_start = large.contains(small.start());
                                                     let contains_end   = large.contains(small.end());

                                                     (one + (contains_start && contains_end) as u32,
                                                      two + (contains_start || contains_end) as u32)
                                                 });

    println!("{one}");
    println!("{two}");
}
