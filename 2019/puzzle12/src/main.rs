use std::cmp::Ordering;

fn main()
{
    let input : Vec<_> = include_str!("../input.txt").lines().map(parse_moon).collect();

    let mut moons  = input.clone();
    let mut cycles = [None, None, None];

    for i in 1 ..= 1000
    {
        step(&mut moons);
        check_cycles(i, &input, &moons, &mut cycles);
    }
    println!("{}", moons.iter().map(|(p, v)| p.iter().map(|x| x.abs()).sum::<i64>()
                                           * v.iter().map(|x| x.abs()).sum::<i64>()).sum::<i64>());

    for i in 1001 ..
    {
        if let [Some(a), Some(b), Some(c)] = cycles
        {
            println!("{}", lcm([a,b,c].iter().copied()));
            break
        }

        step(&mut moons);
        check_cycles(i, &input, &moons, &mut cycles);
    }
}

fn parse_moon(s : &str) -> ([i64 ; 3], [i64 ; 3])
{
    fn span_integer(s : &str) -> (&str, &str)
    {
        s.split_at(s.find(|c : char| !(c.is_ascii_digit() || c == '-')).unwrap_or(s.len()))
    }

    let (x, s) = span_integer(&s[3..]);
    let (y, s) = span_integer(&s[4..]);
    let (z, _) = span_integer(&s[4..]);

    ([x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap()], [0, 0, 0])
}

fn step(moons : &mut [([i64 ; 3], [i64 ; 3])])
{
    for i in 0 .. moons.len()
    {
        for j in i+1 .. moons.len()
        {
            let g = moons[i].0.iter().zip(moons[j].0.iter()).map(|(a, b)|
            {
                match a.cmp(b)
                {
                    Ordering::Less    => -1,
                    Ordering::Equal   =>  0,
                    Ordering::Greater =>  1
                }
            });

            moons[i].1.iter_mut().zip(g.clone()).for_each(|(a, b)| *a -= b);
            moons[j].1.iter_mut().zip(g        ).for_each(|(a, b)| *a += b);
        }

        let (pos, vel) = &mut moons[i];
        pos.iter_mut().zip(vel.iter()).for_each(|(p, v)| *p += *v)
    }
}

fn check_cycles(i : u64, input : &[([i64 ; 3], [i64 ; 3])], moons : &[([i64 ; 3], [i64 ; 3])], cycles : &mut [Option<u64> ; 3])
{
    for j in 0 .. 3
    {
        if cycles[j].is_none() && input.iter().zip(moons.iter()).all(|((pa, va), (pb, vb))| pa[j] == pb[j] && va[j] == vb[j])
        {
            cycles[j] = Some(i);
        }
    }
}

fn lcm(mut values : impl Iterator<Item = u64>) -> u64
{
    match values.next()
    {
        None    => 0,
        Some(x) =>
        {
            let mut lcm_c = x;
            for y in values
            {
                let mut a = lcm_c;
                let mut b = y;
                while b > 0
                {
                    let temp = a;
                    a = b;
                    b = temp % b;
                }

                lcm_c = (lcm_c * y) / a;
            }
            lcm_c
        }
    }
}
