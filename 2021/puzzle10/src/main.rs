fn main()
{
    let (ok, err) : (Vec<_>, Vec<_>) = include_str!("../input.txt").lines().map(check).partition(Result::is_ok);

    println!("{}", err.into_iter().fold(0, |a, b| a + match b.unwrap_err()
    {
        b')' =>     3,
        b']' =>    57,
        b'}' =>  1197,
        b'>' => 25137,
        _    => unreachable!()
    }));

    let mut scores = ok.into_iter().map(|r| r.unwrap().into_iter().rev().fold(0, |a, b| 5*a + match b
    {
        b'(' => 1,
        b'[' => 2,
        b'{' => 3,
        b'<' => 4,
        _    => unreachable!()
    }))
    .collect::<Vec<u64>>();

    let mid = scores.len() / 2;
    println!("{}", scores.select_nth_unstable(mid).1);
}

fn check(s : &str) -> Result<Vec<u8>, u8>
{
    let mut stack = Vec::new();

    for b in s.bytes()
    {
        let c = match b
        {
            b'(' | b'[' | b'{' | b'<' =>
            {
                stack.push(b);
                continue
            }
            b')' => b'(',
            b']' => b'[',
            b'}' => b'{',
            b'>' => b'<',
            _    => unreachable!()
        };

        if Some(c) != stack.pop()
        {
            return Err(b)
        }
    }

    Ok(stack)
}
