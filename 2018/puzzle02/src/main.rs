use itertools::Itertools;

fn main()
{
    let input = include_str!("../input.txt");

    let (x, y) = input.lines().fold((0, 0), |(x, y), s|
    {
        let (a, b) = count(s);
        (if a { x+1 } else { x }, if b { y+1 } else { y })
    });
    println!("{}", x * y);

    'outer: for (s, t) in input.lines().tuple_combinations()
    {
        let mut diff = None;
        for (i, (c, d)) in s.bytes().zip(t.bytes()).enumerate()
        {
            if c != d
            {
                if diff.is_none()
                {
                    diff = Some(i);
                }
                else
                {
                    continue 'outer
                }
            }
        }

        if let Some(i) = diff
        {
            println!("{}{}", &s[..i], &s[i+1..]);
            break
        }
    }
}

fn count(s : &str) -> (bool, bool)
{
    let mut histogram = [0 ; 26];
    s.bytes().for_each(|c| histogram[(c - b'a') as usize] += 1);

    histogram.iter().try_fold((false, false), |(a, b), x|
    {
        match x
        {
            2 => if b { Err((true, true)) } else { Ok((true, b)) },
            3 => if a { Err((true, true)) } else { Ok((a, true)) },
            _ => Ok((a, b))
        }
    })
    .unwrap_or_else(|x| x)
}
