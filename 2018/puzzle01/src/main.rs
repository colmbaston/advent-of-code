use std::collections::HashSet;

fn main()
{
    let input = include_str!("../input.txt").lines().map(|x| x.parse().unwrap()).collect::<Vec<i32>>();

    println!("{}", input.iter().sum::<i32>());

    let mut visited = HashSet::new();
    for x in input.iter().cycle().scan(0, |a, y| { let z = *a; *a += y; Some(z) })
    {
        if !visited.insert(x)
        {
            println!("{}", x);
            break
        }
    }
}
