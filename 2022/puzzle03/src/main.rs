fn main()
{
    let rucksacks = include_str!("../input.txt").lines()
                                                .map(|l| parse_rucksack(l.as_bytes()))
                                                .collect::<Vec<(u64, u64)>>();

    println!("{}", rucksacks.iter()
                            .map(|(a, b)| (a & b).ilog2())
                            .sum::<u32>());
    println!("{}", rucksacks.chunks(3)
                            .map(|arr| arr.iter()
                                          .map(|(a, b)|  a | b)
                                          .reduce(|a, b| a & b)
                                          .unwrap()
                                          .ilog2())
                            .sum::<u32>());
}

fn parse_rucksack(s : &[u8]) -> (u64, u64)
{
    let (a, b) = s.split_at(s.len() / 2);
    (to_bitset(a), to_bitset(b))
}

fn to_bitset(s : &[u8]) -> u64
{
    s.iter().fold(0, |set, b| set | 1 << ((b - 38) % 58))
}
