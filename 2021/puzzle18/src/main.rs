use std::ops::Add;

fn main()
{
    let input = include_str!("../input.txt").lines().map(|mut s| Tree::parse(&mut s)).collect::<Vec<Tree>>();

    let mut sum;
    let mut sum_ref = &input[0];
    for t in input.iter().skip(1)
    {
        sum     =  sum_ref + t;
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
enum Tree
{
    Leaf(u32),
    Node(Box<Tree>, Box<Tree>)
}

impl Add for &Tree
{
    type Output = Tree;

    fn add(self, other : &Tree) -> Tree
    {
        let mut sum = Tree::Node(Box::new(self.clone()), Box::new(other.clone()));
        sum.reduce();
        sum
    }
}

impl Tree
{
    fn parse(s : &mut &str) -> Tree
    {
        let (a, b) = s.split_at(s.find(|c : char| !c.is_ascii_digit()).unwrap());
        *s         = b;

        if a.is_empty()
        {
            *s        = s.strip_prefix('[').unwrap();
            let left  = Tree::parse(s);
            *s        = s.strip_prefix(',').unwrap();
            let right = Tree::parse(s);
            *s        = s.strip_prefix(']').unwrap();

            Tree::Node(Box::new(left), Box::new(right))
        }
        else
        {
            Tree::Leaf(a.parse().unwrap())
        }
    }

    fn magnitude(&self) -> u32
    {
        match self
        {
            Tree::Leaf(k)    => *k,
            Tree::Node(l, r) => 3 * l.magnitude() + 2 * r.magnitude()
        }
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
        match self
        {
            Tree::Leaf(_)    => None,
            Tree::Node(l, r) =>
            {
                if depth == 4
                {
                    Some((Some(l.magnitude()), Some(r.magnitude())))
                }
                else
                {
                    match l.explode(depth+1)
                    {
                        Some((ml, mr)) =>
                        {
                            if ml.is_some() && mr.is_some() { **l = Tree::Leaf(0) }
                            if let Some(k) = mr             { *r.leftmost() += k  }

                            Some((ml, None))
                        }
                        None => r.explode(depth+1).map(|(ml, mr)|
                        {
                            if ml.is_some() && mr.is_some() { **r = Tree::Leaf(0) }
                            if let Some(k) = ml             { *l.rightmost() += k }

                            (None, mr)
                        })
                    }
                }
            }
        }
    }

    fn leftmost(&mut self) -> &mut u32
    {
        match self
        {
            Tree::Leaf(k)    => k,
            Tree::Node(l, _) => l.leftmost()
        }
    }

    fn rightmost(&mut self) -> &mut u32
    {
        match self
        {
            Tree::Leaf(k)    => k,
            Tree::Node(_, r) => r.rightmost()
        }
    }

    fn split(&mut self) -> bool
    {
        match self
        {
            Tree::Leaf(k) => *k >= 10 &&
            {
                let q = *k / 2;
                *self = Tree::Node(Box::new(Tree::Leaf(q)), Box::new(Tree::Leaf(*k - q)));
                true
            },
            Tree::Node(l, r) => l.split() || r.split()
        }
    }
}
