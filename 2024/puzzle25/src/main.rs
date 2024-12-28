fn main()
{
    let (locks, keys) : (Vec<u64>, Vec<u64>) = include_str!("../input.txt").split("\n\n").map(parse_schematic).partition(|s| s & 1 == 1);
    println!("{}", locks.iter().flat_map(|l| keys.iter().map(move |k| (l, k))).filter(|&(l, k)| l & k == 0).count());
}

fn parse_schematic(s : &str) -> u64
{
    s.lines()
     .flat_map(str::bytes)
     .fold(0, |a, b| (a << 1) | (b == b'#') as u64)
}
