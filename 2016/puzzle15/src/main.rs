fn main()
{
    let mut discs = include_str!("../input.txt").lines().map(Disc::parse).collect::<Vec<Disc>>();
    println!("{}", Disc::drop_time(&discs));
    discs.push(Disc { num: discs.len()+1, len: 11, pos: 0 });
    println!("{}", Disc::drop_time(&discs));
}

struct Disc
{
    num: usize,
    len: usize,
    pos: usize
}

impl Disc
{
    fn parse(s : &str) -> Disc
    {
        let (num, s)   = s.strip_prefix("Disc #").unwrap()
                          .split_once(" has ").unwrap();
        let (len, pos) = s.strip_suffix('.').unwrap()
                          .split_once(" positions; at time=0, it is at position ").unwrap();

        Disc
        {
            num: num.parse().unwrap(),
            len: len.parse().unwrap(),
            pos: pos.parse().unwrap()
        }
    }

    fn drop_time(discs : &[Disc]) -> usize
    {
        (0 ..).find(|i| discs.iter().all(|d| (d.num + d.pos + i) % d.len == 0)).unwrap()
    }
}
