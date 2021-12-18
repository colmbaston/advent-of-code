use std::ops::Add;

fn main()
{
    let input = include_str!("../input.txt").lines().map(|mut s| NumberPair::parse(&mut s)).collect::<Vec<NumberPair>>();

    let mut sum;
    let mut sum_ref = &input[0];
    for n in input.iter().skip(1)
    {
        sum     = sum_ref + n;
        sum_ref = &sum;
    }
    println!("{}", sum_ref.magnitude());

    let mut max = u32::MIN;
    for (i, a) in input.iter().enumerate()
    {
        for b in input.iter().skip(i+1)
        {
            max = max.max((a + b).magnitude()).max((b + a).magnitude());
        }
    }
    println!("{}", max);
}

#[derive(Clone)]
struct NumberPair
{
    left  : NumberField,
    right : NumberField
}

#[derive(Clone)]
enum NumberField
{
    Regular(u32),
    Nested(Box<NumberPair>)
}

impl Add for &NumberPair
{
    type Output = NumberPair;

    fn add(self, other : &NumberPair) -> NumberPair
    {
        let left    = NumberField::Nested(Box::new(self.clone()));
        let right   = NumberField::Nested(Box::new(other.clone()));
        let mut sum = NumberPair { left, right };

        sum.reduce();
        sum
    }
}

impl NumberPair
{
    fn parse(s : &mut &str) -> NumberPair
    {
        *s        = s.strip_prefix('[').unwrap();
        let left  = NumberField::parse(s);
        *s        = s.strip_prefix(',').unwrap();
        let right = NumberField::parse(s);
        *s        = s.strip_prefix(']').unwrap();

        NumberPair { left, right }
    }

    fn magnitude(&self) -> u32
    {
        3 * self.left.magnitude() + 2 * self.right.magnitude()
    }

    fn reduce(&mut self)
    {
        loop
        {
            if !(self.explode(0).is_some() || self.split()) { break }
        }
    }

    fn explode(&mut self, depth : u8) -> Option<(Option<u32>, Option<u32>)>
    {
        if depth == 4
        {
            Some((Some(self.left.magnitude()), Some(self.right.magnitude())))
        }
        else
        {
            match self.left.explode(depth+1)
            {
                Some((ml, mr)) =>
                {
                    if ml.is_some() && mr.is_some() {  self.left = NumberField::Regular(0) }
                    if let Some(r) = mr             { *self.right.leftmost() += r          }

                    Some((ml, None))
                }
                None => self.right.explode(depth+1).map(|(ml, mr)|
                {
                    if ml.is_some() && mr.is_some() {  self.right = NumberField::Regular(0) }
                    if let Some(l) = ml             { *self.left.rightmost() += l           }

                    (None, mr)
                })
            }
        }
    }

    fn leftmost(&mut self) -> &mut u32
    {
        self.left.leftmost()
    }

    fn rightmost(&mut self) -> &mut u32
    {
        self.right.rightmost()
    }

    fn split(&mut self) -> bool
    {
        self.left.split() || self.right.split()
    }
}

impl NumberField
{
    fn parse(s : &mut &str) -> NumberField
    {
        let (a, b) = s.split_at(s.find(|c : char| !c.is_ascii_digit()).unwrap());
        *s         = b;

        match a
        {
            "" => NumberField::Nested(Box::new(NumberPair::parse(s))),
            _  => NumberField::Regular(a.parse().unwrap())
        }
    }

    fn magnitude(&self) -> u32
    {
        match self
        {
            NumberField::Regular(r) => *r,
            NumberField::Nested(p)  => p.magnitude()
        }
    }

    fn explode(&mut self, depth : u8) -> Option<(Option<u32>, Option<u32>)>
    {
        match self
        {
            NumberField::Regular(_) => None,
            NumberField::Nested(p)  => p.explode(depth)
        }
    }

    fn leftmost(&mut self) -> &mut u32
    {
        match self
        {
            NumberField::Regular(r) => r,
            NumberField::Nested(p)  => p.leftmost()
        }
    }

    fn rightmost(&mut self) -> &mut u32
    {
        match self
        {
            NumberField::Regular(r) => r,
            NumberField::Nested(p)  => p.rightmost()
        }
    }

    fn split(&mut self) -> bool
    {
        match self
        {
            NumberField::Regular(r) => *r >= 10 &&
            {
                let q     = *r / 2;
                let left  = NumberField::Regular(q);
                let right = NumberField::Regular(*r - q);
                *self     = NumberField::Nested(Box::new(NumberPair { left, right }));

                true
            },
            NumberField::Nested(p) => p.split()
        }
    }
}
