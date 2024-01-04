fn main()
{
    let input = include_str!("../input.txt").lines()
                                            .map(|s| s.bytes().map(|b| b == b'1').collect())
                                            .collect::<Vec<Vec<bool>>>();

    let mut gamma = gamma_rate(&input);
    println!("{}", to_decimal(&gamma) * to_decimal(&gamma.iter().map(|b| !b).collect::<Vec<bool>>()));

    let mut oxygen = input.clone();
    let mut carbon = input;
    for i in 0 .. oxygen[0].len()
    {
        if oxygen.len() > 1
        {
            gamma = gamma_rate(&oxygen);
            oxygen.retain(|bs| bs[i] == gamma[i]);
        }

        if carbon.len() > 1
        {
            gamma = gamma_rate(&carbon);
            carbon.retain(|bs| bs[i] != gamma[i]);
        }
    }
    println!("{}", to_decimal(&oxygen[0]) * to_decimal(&carbon[0]));
}

fn gamma_rate(ns : &[Vec<bool>]) -> Vec<bool>
{
    let mut hist = vec![0 ; ns[0].len()];

    for bs in ns.iter()
    {
        for (&b, n) in bs.iter().zip(hist.iter_mut())
        {
            if b { *n += 1 }
        }
    }

    let half = (ns.len()+1) / 2;
    hist.into_iter().map(|n| half <= n).collect()
}

fn to_decimal(bs : &[bool]) -> u32
{
    bs.iter().fold(0, |a, b| a * 2 + *b as u32)
}
