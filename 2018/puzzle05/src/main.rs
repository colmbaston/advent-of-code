fn main()
{
    let polymer = include_str!("../input.txt").trim_end();
    println!("{}", react(polymer.bytes()));
    println!("{}", (b'a' ..= b'z').map(|c| react(polymer.bytes().filter(|d| c != d.to_ascii_lowercase())))
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
            Some(&d) if c != d && c.eq_ignore_ascii_case(&d) => { v.pop();  },
            _                                                => { v.push(c) }
        }
    }
    v.len()
}
