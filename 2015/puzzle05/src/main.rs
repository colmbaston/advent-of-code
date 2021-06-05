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
    for w in s.as_bytes().windows(2)
    {
        if BANNED.contains(&w) { return false }
        if w[0] == w[1]        { twice = true }
    }
    twice
}

fn nice_two(s : &str) -> bool
{
    let mut found = false;
    for w in s.as_bytes().windows(3)
    {
        if w[0] == w[2] { found = true; break }
    }

    if !found { return false }
    found = false;

    'outer: for i in 0 ..= s.len() - 4
    {
        let (a, b) = s.as_bytes()[i..].split_at(2);
        for j in 0 ..= b.len() - 2
        {
            if a == &b[j .. j+2] { found = true; break 'outer }
        }
    }

    found
}
