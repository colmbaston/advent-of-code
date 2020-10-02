const INPUT : &str = include_str!("../input.txt");

fn main()
{
    println!("{}", react(INPUT.bytes()));
    println!("{}", (b'a' ..= b'z').map(|c| react(INPUT.bytes().filter(|d| c != d.to_ascii_lowercase())))
                                  .min()
                                  .unwrap());
}

fn react(polymer : impl Iterator<Item = u8>) -> usize
{
    let mut v = Vec::new();
    for c in polymer
    {
        match v.last()
        {
            Some(&d) if c != d && c.to_ascii_lowercase() == d.to_ascii_lowercase() => { v.pop();  },
            _                                                                      => { v.push(c) }
        }
    }
    v.len()
}
