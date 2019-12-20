use parsing::*;
use std::cmp::Ordering;
use num_integer::Integer;

fn main()
{
    let input : Vec<_> = include_str!("../input.txt").lines().map(|s| parse_moon(s).unwrap().1).collect();

    let mut moons  = input.clone();
    let mut cycles = [None, None, None];

    for i in 1 .. 1001
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
            println!("{}", a.lcm(&b).lcm(&c));
            break
        }

        step(&mut moons);
        check_cycles(i, &input, &moons, &mut cycles);
    }
}

fn parse_moon(s : &str) -> IResult<&str, ([i64 ; 3], [i64 ; 3])>
{
    let (s, (_, x, _, y, _, z)) = tuple((tag("<x="), integer, tag(", y="), integer, tag(", z="), integer))(s)?;
    Ok((s, ([x, y, z], [0, 0, 0])))
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
        moons[i].0.iter_mut().zip(moons[i].1.iter()).for_each(|(a, b)| *a += b);
    }
}

fn check_cycles(i : u64, input : &[([i64 ; 3], [i64 ; 3])], moons :  &[([i64 ; 3], [i64 ; 3])], cycles : &mut [Option<u64> ; 3])
{
    for j in 0 .. 3
    {
        if cycles[j].is_none() && input.iter().zip(moons.iter()).all(|((pa, va), (pb, vb))| pa[j] == pb[j] && va[j] == vb[j])
        {
                cycles[j] = Some(i);
        }
    }
}
