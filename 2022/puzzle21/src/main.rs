use std::collections::HashMap;

fn main()
{
    let monkeys = include_str!("../input.txt").lines().filter_map(|l|
    {
        let mut split = l.split(": ");
        Some((split.next()?, Expr::parse(split.next()?)?))
    })
    .collect::<HashMap<&str, Expr>>();

    if let Some(root) = Expr::Var("root").eval(&monkeys, &mut HashMap::new())
    {
        println!("{root}")
    }
}

enum Expr<'a>
{
    Int(i64),
    Var(&'a str),
    Op(Op, Box<Expr<'a>>, Box<Expr<'a>>)
}

#[derive(Clone, Copy)]
enum Op { Add, Sub, Mul, Div }

impl<'a> Expr<'a>
{
    fn parse(s : &str) -> Option<Expr>
    {
        let mut words = s.split_whitespace();

        let lhs = words.next()?;
        let lhs = lhs.parse::<i64>().map_or(Expr::Var(lhs), Expr::Int);

        let op = match words.next()
        {
            None     => return Some(lhs),
            Some(op) => Op::parse(op)?
        };

        let rhs = words.next()?;
        let rhs = rhs.parse::<i64>().map_or(Expr::Var(rhs), Expr::Int);

        Some(Expr::Op(op, Box::new(lhs), Box::new(rhs)))
    }

    fn eval(&self, context : &HashMap<&'a str, Expr<'a>>, cache : &mut HashMap<&'a str, i64>) -> Option<i64>
    {
        match self
        {
            &Expr::Int(k) => Some(k),
            &Expr::Var(v)  => cache.get(v).copied().or_else(||
            {
                let k = context.get(v)?.eval(context, cache)?;
                cache.insert(v, k);
                Some(k)
            }),
            Expr::Op(op, lhs, rhs) => Some(op.eval(lhs.eval(context, cache)?,
                                                   rhs.eval(context, cache)?))
        }
    }
}

impl Op
{
    fn parse(s : &str) -> Option<Op>
    {
        match s
        {
            "+" => Some(Op::Add),
            "-" => Some(Op::Sub),
            "*" => Some(Op::Mul),
            "/" => Some(Op::Div),
            _   => None
        }
    }

    fn eval(self, x : i64, y : i64) -> i64
    {
        match self
        {
            Op::Add => x + y,
            Op::Sub => x - y,
            Op::Mul => x * y,
            Op::Div => x / y
        }
    }
}
