fn main()
{
    let mut ops = Vec::new();
    println!("{}", include_str!("../input.txt").trim_end()
                                               .split(',')
                                               .map(|s| { ops.push(Op::parse(s)); hash(s) as u32 })
                                               .sum::<u32>());

    let mut boxes = vec![Vec::new() ; 256];
    for op in ops.into_iter()
    {
        match op
        {
            Op::Remove(label) =>
            {
                boxes[hash(label) as usize].retain(|(l, _)| l != &label)
            },
            Op::Insert(label, lens) =>
            {
                let lenses = &mut boxes[hash(label) as usize];
                match lenses.iter_mut().find(|(l, _)| l == &label)
                {
                    None            => lenses.push((label, lens)),
                    Some((_, slot)) => *slot = lens
                }
            }
        }
    }

    println!("{}", boxes.into_iter().zip(1..)
                        .flat_map(|(ls, box_num)| ls.into_iter().zip(1..)
                                                    .map(move |((_, lens), slot)| box_num * slot * lens as u32))
                        .sum::<u32>());
}

fn hash(s : &str) -> u8
{
    s.bytes().fold(0, |a, b| a.wrapping_add(b).wrapping_mul(17))
}

enum Op<'a>
{
    Remove(&'a str),
    Insert(&'a str, u8)
}

impl Op<'_>
{
    fn parse(s : &str) -> Op<'_>
    {
        let (label, s) = s.split_at(s.find(['-', '=']).unwrap());
        let (op,    s) = s.split_at(1);

        match op
        {
            "-" => Op::Remove(label),
            "=" => Op::Insert(label, s.parse().unwrap()),
            _   => unreachable!()
        }
    }
}
