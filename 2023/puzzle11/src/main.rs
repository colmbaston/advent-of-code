use std::collections::HashSet;

fn main()
{
    let (galaxies, empty_x, empty_y) = parse_galaxies(include_str!("../input.txt"));
    println!("{}", manhattan_sum(&expand(&galaxies, &empty_x, &empty_y,         2)));
    println!("{}", manhattan_sum(&expand(&galaxies, &empty_x, &empty_y, 1_000_000)));
}

type Pos = (u64, u64);

fn parse_galaxies(s : &str) -> (Vec<Pos>, HashSet<u64>, HashSet<u64>)
{
    let mut galaxies = Vec::new();
    let mut empty_x  = (0 .. s.lines().next().unwrap().len() as u64).collect::<HashSet<u64>>();
    let mut empty_y  = (0 .. s.lines().count()               as u64).collect::<HashSet<u64>>();

    for (l, y) in s.lines().zip(0..)
    {
        for (b, x) in l.bytes().zip(0..)
        {
            if let b'#' = b
            {
                galaxies.push((x, y));
                empty_x.remove(&x);
                empty_y.remove(&y);
            }
        }
    }

    (galaxies, empty_x, empty_y)
}

fn expand(galaxies : &[Pos], empty_x : &HashSet<u64>, empty_y : &HashSet<u64>, factor : u64) -> Vec<Pos>
{
    let mut expanded = galaxies.to_vec();
    for (x, y) in expanded.iter_mut()
    {
        *x += (factor-1) * empty_x.iter().filter(|&ex| ex < x).count() as u64;
        *y += (factor-1) * empty_y.iter().filter(|&ey| ey < y).count() as u64;
    }
    expanded
}

fn manhattan_sum(galaxies : &[Pos]) -> u64
{
    let mut sum = 0;
    for (i, &(x1, y1)) in galaxies.iter().enumerate()
    {
        for &(x2, y2) in galaxies[i+1..].iter()
        {
            sum += x1.abs_diff(x2) + y1.abs_diff(y2);
        }
    }
    sum
}
