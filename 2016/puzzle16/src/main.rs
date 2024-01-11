#![feature(array_chunks)]

fn main()
{
    let mut curve = include_str!("../input.txt").trim_end().bytes()
                                                .map(|b| b == b'1')
                                                .collect::<Vec<bool>>();

    let mut buffer = Vec::new();
    while curve.len() < 35_651_584
    {
        buffer.clear();
        buffer.extend(curve.iter());
        buffer.push(false);
        buffer.extend(curve.iter().rev().map(|b| !b));
        std::mem::swap(&mut curve, &mut buffer);
    }

    // the factorisations of 272 and 35_651_584 are both 2^i * 17
    println!("{:017b}", checksum(&curve[..        272]));
    println!("{:017b}", checksum(&curve[.. 35_651_584]));
}

fn checksum(bits : &[bool]) -> u32
{
    let digest = |s : &[bool]| s.array_chunks()
                                .map(|[a, b]| a == b)
                                .collect::<Vec<bool>>();

    let mut sum = digest(bits);
    while sum.len() % 2 == 0 { sum = digest(&sum) }
    sum.into_iter().fold(0, |a, b| a << 1 | b as u32)
}
