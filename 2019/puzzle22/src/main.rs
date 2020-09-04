use parsing::*;
use std::ops::RemAssign;

const DECK_ONE : i128 = 10_007;
const DECK_TWO : i128 = 119_315_717_514_047;
const SHUFFLES : u64  = 101_741_582_076_661;

fn main()
{
    let input : Vec<Linear> = include_str!("../input.txt").lines().map(|s|
    {
        alt((map(tag("deal into new stack"),                     |_| Linear { a: -1,         b: -1         }),
             map(preceded(tag("cut "),                 integer), |c| Linear { a:  1,         b: -c as i128 }),
             map(preceded(tag("deal with increment "), integer), |i| Linear { a:  i as i128, b:  0         })))(s).unwrap().1
    })
    .collect();

    let mut shuffle = input.iter().fold(Linear::IDENTITY, |mut a, x| { a.compose(x); a %= DECK_ONE; a });
    shuffle.modulus(DECK_ONE);
    println!("{}", (shuffle.a * 2019 + shuffle.b) % DECK_ONE);

    let mut shuffle = input.iter().fold(Linear::IDENTITY, |mut a, x| { a.compose(x); a %= DECK_TWO; a });
    shuffle.exp_by_squaring_mod(SHUFFLES, DECK_TWO);
    let inverse = egcd(shuffle.a, DECK_TWO).1;
    println!("{:?}", ((inverse * 2020 - inverse * shuffle.b) % DECK_TWO + DECK_TWO) % DECK_TWO);
}

#[derive(Clone)]
struct Linear
{
    a : i128,
    b : i128
}

impl Linear
{
    const IDENTITY : Linear = Linear { a: 1, b: 0 };

    fn compose(&mut self, other : &Linear)
    {
        *self = Linear { a: self.a * other.a, b: other.a * self.b + other.b }
    }

    fn exp_by_squaring_mod(&mut self, mut exponent : u64, n : i128)
    {
        let mut x = Linear::IDENTITY;
        while exponent > 1
        {
            if exponent % 2 == 1
            {
                x.compose(&self);
                x %= n;
            }
            self.compose(&self.clone());
            *self %= n;
            exponent /= 2;
        }
        self.compose(&x);
        self.modulus(n)
    }

    fn modulus(&mut self, n : i128)
    {
        self.a = (self.a % n + n) % n;
        self.b = (self.b % n + n) % n;
    }
}

impl RemAssign<i128> for Linear
{
    fn rem_assign(&mut self, n : i128)
    {
        self.a %= n;
        self.b %= n;
    }
}

fn egcd(a : i128, b : i128) -> (i128, i128, i128)
{
    if a == 0
    {
        (b, 0, 1)
    }
    else
    {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}
