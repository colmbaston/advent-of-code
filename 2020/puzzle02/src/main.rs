fn main()
{
    let input = include_str!("../input.txt").lines().map(|x| parse_policy(x).unwrap()).collect::<Vec<_>>();

    println!("{}", input.iter().filter(|(x, y)| valid_one(*x, y)).count());
    println!("{}", input.iter().filter(|(x, y)| valid_two(*x, y)).count());
}

fn parse_policy(s : &str) -> Option<((u8, u8, char), &str)>
{
    fn natural(s : &str) -> Option<(u8, &str)>
    {
        let (n, t) = s.split_at(s.find(|c : char| !c.is_ascii_digit()).unwrap_or_else(|| s.len()));
        n.parse().ok().map(|k| (k, t))
    }

    let (l, s) = natural(s)?;
    let (u, s) = natural(&s[1..])?;

    Some(((l, u, s.chars().nth(1)?), &s[4..]))
}

fn valid_one((l, u, c) : (u8, u8, char), s : &str) -> bool
{
    let occurrences = s.chars().filter(|d| *d == c).count() as u8;

    l <= occurrences && occurrences <= u
}

fn valid_two((l, u, c) : (u8, u8, char), s : &str) -> bool
{
    let l = s.chars().nth(l as usize - 1).map(|d| d == c);
    let u = s.chars().nth(u as usize - 1).map(|d| d == c);

    l.filter(|b| *b).xor(u.filter(|b| *b)).is_some()
}
