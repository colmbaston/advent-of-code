#![feature(array_windows)]

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
    for &[a, b, c] in s.array_windows()
    {
        if a+1 == b && b+1 == c
        {
            found = true;
            break
        }
    }
    if !found { return false }

    for (i, &[a, b]) in s.array_windows().enumerate()
    {
        if a == b
        {
            for &[c, d] in s[i+2 ..].array_windows()
            {
                if c == d { return true }
            }
            return false
        }
    }
    false
}
