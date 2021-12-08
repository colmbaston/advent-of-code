fn main()
{
    let mut input = include_str!("../input.txt").lines().map(parse_entry).collect::<Vec<_>>();

    println!("{}", input.iter().flat_map(|(_, d)| d.iter()).filter(|d| matches!(d.count_ones(), 2 | 3 | 4 | 7)).count());
    println!("{}", input.iter_mut().map(decode).sum::<u32>());
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

fn decode((signals, output) : &mut (Vec<u8>, Vec<u8>)) -> u32
{
    signals.sort_by_key(|b| b.count_ones());
    let one  = signals[0];
    let four = signals[2];

    output.iter().fold(0, |a, b|
    {
        10*a + match (b.count_ones(), (b & one).count_ones(), (b & four).count_ones())
        {
            (2, _, _) => 1,
            (3, _, _) => 7,
            (4, _, _) => 4,
            (5, 2, _) => 3,
            (5, _, 2) => 2,
            (5, _, _) => 5,
            (6, 1, _) => 6,
            (6, _, 4) => 9,
            (6, _, _) => 0,
            _         => 8
        }
    })
}
