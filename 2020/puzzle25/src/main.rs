fn main()
{
    let (card, door) = parse_keys(include_str!("../input.txt"));

    let card_loop = transform(7).enumerate().find(|(_, v)| *v == card).unwrap().0;
    println!("{}", transform(door).nth(card_loop).unwrap());
}

fn parse_keys(s : &str) -> (u64, u64)
{
    let mut it = s.lines();

    (it.next().unwrap().parse().unwrap(),
     it.next().unwrap().parse().unwrap())
}

fn transform(subject : u64) -> impl Iterator<Item = u64>
{
    let mut value = 1;
    std::iter::from_fn(move ||
    {
        value = (value * subject) % 20201227;
        Some(value)
    })
}
