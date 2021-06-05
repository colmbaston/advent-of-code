fn main()
{
    let mut input = include_str!("../input.txt").trim_end().bytes().collect::<Vec<u8>>();

    for _ in 0 .. 2
    {
        succ(&mut input);
        while !valid(&input) { succ(&mut input) }
        println!("{}", std::str::from_utf8(&input).unwrap());
    }
}

fn succ(s : &mut [u8])
{
    let (last, init) = s.split_last_mut().unwrap();

    match last
    {
        b'z' => { *last  = b'a'; succ(init) },
        _    =>   *last += 1
    }
}

fn valid(s : &[u8]) -> bool
{
    for b in s.iter()
    {
        if let b'i' | b'o' | b'l' = b { return false }
    }

    let mut found = false;

    for w in s.windows(3)
    {
        if w[0]+1 == w[1] && w[1]+1 == w[2]
        {
            found = true; break;
        }
    }

    if !found { return false }
    found = false;

    'outer: for (i, w) in s.windows(2).enumerate()
    {
        if w[0] == w[1]
        {
            for v in s[i+2 ..].windows(2)
            {
                if v[0] == v[1] { found = true; break 'outer }
            }
        }
    }

    found
}
