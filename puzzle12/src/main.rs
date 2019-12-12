use num::Integer;
use std::cmp::Ordering;

fn main()
{
    let input = vec![(vec![  4, 12, 13], vec![0, 0, 0]),
                     (vec![ -9, 14, -3], vec![0, 0, 0]),
                     (vec![ -7, -1,  2], vec![0, 0, 0]),
                     (vec![-11, 17, -1], vec![0, 0, 0])];

    let mut moons  = input.clone();
    let mut cycles = vec![None, None, None];

    for i in 1 .. 1001
    {
        step(&mut moons);
        check_cycles(i, &input, &moons, &mut cycles);
    }
    println!("{}", moons.iter().map(|(p, v)| p.iter().map(|x| x.abs()).sum::<i64>()
                                           * v.iter().map(|x| x.abs()).sum::<i64>()).sum::<i64>());

    for i in 1001 ..
    {
        if let [Some(a), Some(b), Some(c)] = cycles[..]
        {
            println!("{}", a.lcm(&b).lcm(&c));
            break
        }

        step(&mut moons);
        check_cycles(i, &input, &moons, &mut cycles);
    }
}

fn step(moons : &mut [(Vec<i64>, Vec<i64>)])
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
            })
            .collect::<Vec<_>>();

            moons[i].1 = moons[i].1.iter().zip(g.iter()).map(|(a, b)| a - b).collect();
            moons[j].1 = moons[j].1.iter().zip(g.iter()).map(|(a, b)| a + b).collect();
        }
        moons[i].0 = moons[i].0.iter().zip(moons[i].1.iter()).map(|(a, b)| a + b).collect();
    }
}

fn check_cycles(i : u64, input : &Vec<(Vec<i64>, Vec<i64>)>, moons : &Vec<(Vec<i64>, Vec<i64>)>, cycles : &mut Vec<Option<u64>>)
{
    for j in 0 .. 3
    {
        if let None = cycles[j]
        {
            if input.iter().zip(moons.iter()).all(|((pa, va), (pb, vb))| pa[j] == pb[j] && va[j] == vb[j])
            {
                cycles[j] = Some(i);
            }
        }
    }
}
