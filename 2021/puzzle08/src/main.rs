const DIGITS : &[&str] = &["abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "ace", "abcdefg", "abcdfg"];

fn main()
{
    let input = include_str!("../input.txt").lines().map(parse_entry).collect::<Vec<_>>();

    println!("{}", input.iter()
                        .flat_map(|(_, d)| d.iter())
                        .filter(|d| matches!(d.count_ones(), 2 | 3 | 4 | 7))
                        .count());
}

fn parse_entry(s : &str) -> (Vec<u8>, Vec<u8>)
{
    let mut i = s.split(" | ").map(|t| t.split_whitespace().map(parse_signal).collect());

    (i.next().unwrap(), i.next().unwrap())
}

fn parse_signal(s : &str) -> u8
{
    s.bytes().fold(0, |a, b| a | 64 >> (b - b'a'))
}
