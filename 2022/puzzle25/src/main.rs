use std::cmp::Ordering;

fn main()
{
    let snafu = to_snafu(include_str!("../input.txt").lines()
                                                     .map(|l| from_snafu(l.as_bytes()))
                                                     .sum::<i64>());

    for digit in snafu { print!("{}", digit as char) }
    println!();
}

fn from_snafu(snafu : &[u8]) -> i64
{
    snafu.iter().fold(0, |acc, digit| 5 * acc + match digit
    {
        b'=' => -2,
        b'-' => -1,
        b'0' =>  0,
        b'1' =>  1,
        b'2' =>  2,
        _    => unreachable!()
    })
}

fn to_snafu(k : i64) -> Vec<u8>
{
    let mut snafu = Vec::new();
    let mut abs   = k.unsigned_abs();

    while abs > 0
    {
        let rem = abs % 5;

        snafu.push(match rem
        {
            3 => b'=',
            4 => b'-',
            0 => b'0',
            1 => b'1',
            2 => b'2',
            _ => unreachable!()
        });

        abs = abs / 5 + (rem > 2) as u64
    }

    snafu.reverse();
    match k.cmp(&0)
    {
        Ordering::Less  => negate_snafu(&mut snafu),
        Ordering::Equal => snafu.push(b'0'),
        _               => ()
    }
    snafu
}

fn negate_snafu(snafu : &mut [u8])
{
    for digit in snafu.iter_mut()
    {
        *digit = match *digit
        {
            b'=' => b'2',
            b'-' => b'1',
            b'0' => b'0',
            b'1' => b'-',
            b'2' => b'=',
            _    => unreachable!()
        };
    }
}
