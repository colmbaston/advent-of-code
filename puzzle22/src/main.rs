use parsing::*;

const DECK : i64 = 10_007;

fn main()
{
    let input : Vec<Technique> = include_str!("../input.txt").lines().map(|s|
    {
        alt((map(tag("deal into new stack"),                     |_| Technique::DealIntoNewStack),
             map(preceded(tag("cut "),                 integer), |c| Technique::Cut(c)),
             map(preceded(tag("deal with increment "), integer), |i| Technique::DealWithIncrement(i))))(s).unwrap().1
    })
    .collect();

    println!("{}", (input.iter().fold(2019, |a, t|
    {
        match *t
        {
            Technique::DealIntoNewStack     => (-a - 1) % DECK,
            Technique::Cut(c)               => ( a - c) % DECK,
            Technique::DealWithIncrement(i) => ( a * i) % DECK
        }
    }) + DECK) % DECK);
}

enum Technique
{
    DealIntoNewStack,
    Cut(i64),
    DealWithIncrement(i64)
}
