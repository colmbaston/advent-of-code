fn main()
{
    let input = include_str!("../input.txt");

    println!("{}", input.lines().filter(|&s| nice_one(s)).count());
    println!("{}", input.lines().filter(|&s| nice_two(s)).count());
}

fn nice_one(s : &str) -> bool
{
    const VOWELS : &[u8]    = b"aeiou";
    const BANNED : &[&[u8]] = &[b"ab", b"cd", b"pq", b"xy"];

    if s.bytes().filter(|b| VOWELS.contains(b)).count() < 3 { return false }

    let mut twice = false;
    for &w@[a, b] in s.as_bytes().array_windows()
    {
        if BANNED.contains(&w.as_slice()) { return false }
        if a == b                         { twice = true }
    }
    twice
}

fn nice_two(s : &str) -> bool
{
    let mut found = false;
    for &[a, _, c] in s.as_bytes().array_windows()
    {
        if a == c { found = true; break }
    }
    if !found { return false }

    for i in 0 ..= s.len() - 4
    {
        let (a, b) = s.as_bytes()[i..].split_at(2);
        for j in 0 ..= b.len() - 2
        {
            if a == &b[j .. j+2] { return true }
        }
    }
    false
}
