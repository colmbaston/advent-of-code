use std::collections::{ HashSet, HashMap };

fn main()
{
    let mut input_one = HashSet::new();
    include_str!("../input.txt").bytes().fold((1, 1), |(x, y), b|
    {
        if b == b'#' { input_one.insert((x, y, 0)); }

        if b == b'\n'
        {
            (1, y+1)
        }
        else
        {
            (x+1, y)
        }
    });
    let mut input_two = input_one.clone();

    let mut visited = HashSet::new();
    loop
    {
        let rating = input_one.iter().fold(0, |a, (x, y, _)| a | 1 << (x + 5*y - 6));
        if !visited.insert(rating)
        {
            println!("{}", rating);
            break
        }
        generation(&mut input_one, adjacent);
    }

    (0..200).for_each(|_| generation(&mut input_two, adjacent_rec));
    println!("{}", input_two.len());
}

type Bug = (u8, u8, i8);

fn generation<I>(prev : &mut HashSet<Bug>, adjacent : impl Fn(Bug) -> I)
where I : Iterator<Item = Bug>
{
    let mut adj_count = prev.iter().copied().zip(std::iter::repeat(0)).collect::<HashMap<_,_>>();
    prev.iter().for_each(|b| adjacent(*b).for_each(|b| *adj_count.entry(b).or_insert(0) += 1));

    adj_count.into_iter().for_each(|(k, v)|
    {
        if prev.contains(&k) && v != 1
        {
            prev.remove(&k);
        }
        else if v == 1 || v == 2
        {
            prev.insert(k);
        }
    });
}

fn adjacent((x, y, z) : Bug) -> impl Iterator<Item = Bug>
{
    vec![(x+1, y, z), (x-1, y, z), (x, y+1, z), (x, y-1, z)].into_iter().filter(|&(x, y, _)|
    {
        1 <= x && x <= 5 && 1 <= y && y <= 5
    })
}

fn adjacent_rec((x, y, z) : Bug) -> impl Iterator<Item = Bug>
{
    let mut neighbours = adjacent((x, y, z)).filter(|&(x, y, _)| x != 3 || y != 3).collect::<Vec<_>>();

    if      x == 1           { neighbours.push((2, 3, z-1)) }
    else if x == 5           { neighbours.push((4, 3, z-1)) }
    if      y == 1           { neighbours.push((3, 2, z-1)) }
    else if y == 5           { neighbours.push((3, 4, z-1)) }
    else if x == 3 && y == 2 { neighbours.extend((1..=5).map(|x| (x, 1, z+1))) }
    else if x == 3 && y == 4 { neighbours.extend((1..=5).map(|x| (x, 5, z+1))) }
    else if x == 2 && y == 3 { neighbours.extend((1..=5).map(|y| (1, y, z+1))) }
    else if x == 4 && y == 3 { neighbours.extend((1..=5).map(|y| (5, y, z+1))) }

    neighbours.into_iter()
}
