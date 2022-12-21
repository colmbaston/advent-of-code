use std::collections::HashMap;

fn main()
{
    let mut monkeys = include_str!("../input.txt").lines().filter_map(|l|
    {
        let mut parts = l.split(": ");
        Some((parts.next()?, Expr::parse(parts.next()?)?))
    })
    .collect::<HashMap<&str, Expr>>();

    if let Some(root) = Expr::Var("root").eval(&mut monkeys) { println!("{root}") }
}

#[derive(Debug, Clone)]
enum Expr<'a>
{
    Int(i64),
    Var(&'a str),
    Op(Op, Box<Expr<'a>>, Box<Expr<'a>>)
}

#[derive(Debug, Clone, Copy)]
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

    fn eval(&self, context : &mut HashMap<&'a str, Expr<'a>>) -> Option<i64>
    {
        match self
        {
            Expr::Int(k) => Some(*k),
            Expr::Var(v) => match context.get(v)?.clone()
            {
                Expr::Int(k) => Some(k),
                expr         =>
                {
                    let k = expr.eval(context)?;
                    context.insert(v, Expr::Int(k));
                    Some(k)
                }
            },
            Expr::Op(op, lhs, rhs) => Some(op.eval(lhs.eval(context)?,
                                                   rhs.eval(context)?))
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
