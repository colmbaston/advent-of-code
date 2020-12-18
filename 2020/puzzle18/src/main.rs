fn main()
{
    let input = include_str!("../input.txt").lines().collect::<Vec<_>>();

    println!("{}", input.iter().map(|l| Expr::parse_one(l).unwrap().eval()).sum::<u64>());
    println!("{}", input.iter().map(|l| Expr::parse_two(l).unwrap().eval()).sum::<u64>());
}

enum Expr
{
    Lit(u64),
    Op(Op, Box<Expr>, Box<Expr>)
}

#[derive(Clone)]
enum Op
{
    Add,
    Mul
}

impl Expr
{
    fn eval(&self) -> u64
    {
        match self
        {
            Expr::Lit(k)       => *k,
            Expr::Op(op, x, y) => op.apply(x.eval(), y.eval())
        }
    }

    fn parse_one(s : &str) -> Option<Expr>
    {
        let (x, s) = Expr::parse_op_one(s.trim())?;
        if s.is_empty() { Some(x) } else { None }
    }

    fn parse_op_one(s : &str) -> Option<(Expr, &str)>
    {
        let (mut x, mut s) = Expr::parse_atom_one(s)?;
        let ops = [('+', Op::Add), ('*', Op::Mul)];

        loop
        {
            s = s.trim_start();
            match ops.iter().find_map(|(p, op)| s.strip_prefix(|c| c == *p).map(|s| (op, s)))
            {
                None          => break Some((x, s)),
                Some((op, t)) =>
                {
                    let     t  = t.trim_start();
                    let (y, t) = Expr::parse_atom_one(t)?;

                    x = Expr::Op(op.clone(), Box::new(x), Box::new(y));
                    s = t;
                }
            }
        }
    }

    fn parse_atom_one(s : &str) -> Option<(Expr, &str)>
    {
        match s.strip_prefix('(')
        {
            None =>
            {
                let (ds, s) = s.split_at(s.find(|c : char| !c.is_ascii_digit()).unwrap_or_else(|| s.len()));
                ds.parse().ok().map(|k| (Expr::Lit(k), s))
            }
            Some(s) =>
            {
                let (x, s) = Expr::parse_op_one(s.trim_start())?;
                s.trim_start().strip_prefix(')').map(|s| (x, s))
            }
        }
    }

    fn parse_two(s : &str) -> Option<Expr>
    {
        let (x, s) = Expr::parse_mul_two(s.trim())?;
        if s.is_empty() { Some(x) } else { None }
    }

    fn parse_mul_two(s : &str) -> Option<(Expr, &str)>
    {
        let (x, s) = Expr::parse_add_two(s)?;
        match s.trim_start().strip_prefix('*')
        {
            None    => Some((x, s)),
            Some(s) => Expr::parse_mul_two(s.trim_start()).map(|(y, s)| (Expr::Op(Op::Mul, Box::new(x), Box::new(y)), s))
        }
    }

    fn parse_add_two(s : &str) -> Option<(Expr, &str)>
    {
        let (x, s) = Expr::parse_atom_two(s)?;
        match s.trim_start().strip_prefix('+')
        {
            None    => Some((x, s)),
            Some(s) => Expr::parse_add_two(s.trim_start()).map(|(y, s)| (Expr::Op(Op::Add, Box::new(x), Box::new(y)), s))
        }
    }

    fn parse_atom_two(s : &str) -> Option<(Expr, &str)>
    {
        match s.strip_prefix('(')
        {
            None =>
            {
                let (ds, s) = s.split_at(s.find(|c : char| !c.is_ascii_digit()).unwrap_or_else(|| s.len()));
                ds.parse().ok().map(|k| (Expr::Lit(k), s))
            }
            Some(s) =>
            {
                let (x, s) = Expr::parse_mul_two(s.trim_start())?;
                s.trim_start().strip_prefix(')').map(|s| (x, s))
            }
        }
    }
}

impl Op
{
    fn apply(&self, x : u64, y : u64) -> u64
    {
        match self
        {
            Op::Add => x + y,
            Op::Mul => x * y
        }
    }
}
