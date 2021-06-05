fn main()
{
    let input = include_str!("../input.txt");

    println!("{}", input.lines().map(|s| s.len() - decoded(s)).sum::<usize>());
    println!("{}", input.lines().map(|s| encoded(s) - s.len()).sum::<usize>());
}

fn decoded(s : &str) -> usize
{
    let mut len  = 0;
    let mut skip = 0;

    for b in s.bytes()
    {
        match b
        {
            b'\\' => if skip > 0 { skip -= 1 } else { len += 1; skip += 1 },
            b'\"' => if skip > 0 { skip -= 1 },
            b'x'  => if skip > 0 { skip += 1 } else { len += 1 },
            _     => if skip > 0 { skip -= 1 } else { len += 1 }
        }
    }

    len
}

fn encoded(s : &str) -> usize
{
    2 + s.bytes().map(|b| match b { b'\\' | b'\"' => 2, _ => 1 }).sum::<usize>()
}
