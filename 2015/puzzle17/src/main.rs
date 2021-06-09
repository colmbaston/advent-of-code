use std::cmp::Ordering;

fn main()
{
    let input = include_str!("../input.txt").lines().map(|l| l.parse().unwrap()).collect::<Vec<i32>>();

    let (count, _, min_count) = splits(&input, 150).fold((0, u32::MAX, 0), |(c, m, mc), l|
    {
        match l.cmp(&m)
        {
            Ordering::Less    => (c+1, l, 1),
            Ordering::Equal   => (c+1, m, mc+1),
            Ordering::Greater => (c+1, m, mc)
        }
    });

    println!("{}", count);
    println!("{}", min_count);
}

fn splits(cs : &[i32], m : i32) -> Box<dyn Iterator<Item = u32>>
{
    match m.cmp(&0)
    {
        Ordering::Less    => Box::new(std::iter::empty()),
        Ordering::Equal   => Box::new(std::iter::once(0)),
        Ordering::Greater =>
        {
            match cs.split_first()
            {
                None             => Box::new(std::iter::empty()),
                Some((&k, rest)) => Box::new(splits(rest, m-k).map(|l| l+1).chain(splits(rest, m)))
            }
        }
    }
}
