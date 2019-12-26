use parsing::*;

const DECK_ONE : i64 = 10_007;
const _DECK_TWO : i64 = 119_315_717_514_047;
const _SHUFFLES : i64 = 101_741_582_076_661;

fn main()
{
    let input : Vec<Linear> = include_str!("../input.txt").lines().map(|s|
    {
        alt((map(tag("deal into new stack"),                     |_| Linear { a: -1, b: -1 }),
             map(preceded(tag("cut "),                 integer), |c| Linear { a:  1, b: -c }),
             map(preceded(tag("deal with increment "), integer), |i| Linear { a:  i, b:  0 })))(s).unwrap().1
    })
    .collect();

    let shuffle = input.iter().fold(Linear { a: 1, b: 0 }, |x, y| x.compose_mod(y, DECK_ONE));
    println!("{}", ((shuffle.a * 2019 + shuffle.b) % DECK_ONE + DECK_ONE) % DECK_ONE);
}

struct Linear
{
    a : i64,
    b : i64
}

impl Linear
{
    fn compose_mod(self, other : &Linear, n : i64) -> Linear
    {
        Linear { a: (self.a * other.a) % n, b: (other.a * self.b + other.b) % n }
    }
}
