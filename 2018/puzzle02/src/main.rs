fn main()
{
    let input = include_str!("../input.txt");

    let (x, y) = input.lines().fold((0, 0), |(x, y), s|
    {
        let mut histogram = [0 ; 26];
        s.bytes().for_each(|c| histogram[(c - b'a') as usize] += 1);

        let (a, b) = histogram.iter().try_fold((false, false), |(a, b), x|
        {
            match x
            {
                2 => if b { Err((true, true)) } else { Ok((true, b)) },
                3 => if a { Err((true, true)) } else { Ok((a, true)) },
                _ => Ok((a, b))
            }
        })
        .unwrap_or_else(|x| x);

        (x + a as u32, y + b as u32)
    });
    println!("{}", x * y);

    'outer: for (i, s) in input.lines().enumerate()
    {
        'inner: for t in input.lines().skip(i+1)
        {
            let mut diff = None;
            for (j, (c, d)) in s.bytes().zip(t.bytes()).enumerate()
            {
                if c != d
                {
                    diff = match diff
                    {
                        None => Some(j),
                        _    => continue 'inner
                    }
                }
            }

            if let Some(j) = diff
            {
                println!("{}{}", &s[..j], &s[j+1..]);
                break 'outer
            }
        }
    }
}
