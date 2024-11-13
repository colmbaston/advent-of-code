fn main()
{
    let input = include_str!("../input.txt").lines().map(parse_row).collect::<Vec<[Option<u32> ; COMPOUND_COUNT]>>();
    let cats  = compound_index("cats:");
    let trees = compound_index("trees:");
    let poms  = compound_index("pomeranians:");
    let golds = compound_index("goldfish:");

    println!("{}", input.iter().zip(1..).find(|(cs, _)| matches(cs, |_, a, b| a == b)).unwrap().1);
    println!("{}", input.iter().zip(1..).find(|(cs, _)| matches(cs, |i, a, b|
           if i == cats || i == trees { a >  b }
      else if i == poms || i == golds { a <  b }
      else                            { a == b })).unwrap().1);
}

const COMPOUND_COUNT : usize = 10;

fn parse_row(s : &str) -> [Option<u32> ; COMPOUND_COUNT]
{
    let mut i         = s.split(' ').skip(2);
    let mut compounds = [None ; COMPOUND_COUNT];

    while let (Some(a), Some(b)) = (i.next(), i.next())
    {
        compounds[compound_index(a)] = Some(b.trim_end_matches(',').parse().unwrap())
    }

    compounds
}

fn compound_index(s : &str) -> usize
{
    match s
    {
        "children:"    => 0,
        "cats:"        => 1,
        "samoyeds:"    => 2,
        "pomeranians:" => 3,
        "akitas:"      => 4,
        "vizslas:"     => 5,
        "goldfish:"    => 6,
        "trees:"       => 7,
        "cars:"        => 8,
        "perfumes:"    => 9,
        _              => unreachable!()
    }
}


fn matches(s : &[Option<u32> ; COMPOUND_COUNT], f : impl Fn(usize, u32, u32) -> bool) -> bool
{
    const MFCSAM : [u32 ; COMPOUND_COUNT] = [3, 7, 2, 3, 0, 0, 5, 3, 2, 1];

    s.iter()
     .zip(MFCSAM.iter())
     .enumerate()
     .all(|(i, (a, b))| a.map(|a| f(i, a, *b)).unwrap_or(true))
}
