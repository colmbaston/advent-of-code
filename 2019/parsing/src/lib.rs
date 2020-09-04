pub use nom::{ IResult,
               branch::{ alt },
               bytes::complete::{ tag },
               character::complete::{ alpha1, alphanumeric1, digit1, char, one_of, newline },
               combinator::{ map, opt },
               sequence::{ tuple, preceded, separated_pair },
               multi::{ fold_many0, separated_list }};

pub fn natural(s : &str) -> IResult<&str, u64>
{
    digit1(s).map(|(s, n)| (s, n.parse().unwrap()))
}

pub fn integer(s : &str) -> IResult<&str, i64>
{
    let (s, sign) = opt(one_of("+-"))(s)?;

    let negate = match sign
    {
        Some('-') => true,
        _         => false
    };

    natural(s).map(|(s, n)| (s, if negate { -(n as i64) } else { n as i64 }))
}
