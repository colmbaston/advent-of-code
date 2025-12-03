#![feature(box_patterns)]

use std::collections::HashMap;

fn main()
{
    let mut monkeys = include_str!("../input.txt").lines().filter_map(|l|
    {
        let mut parts = l.split(": ");
        Some((parts.next()?, Expr::parse(parts.next()?)?))
    })
    .collect::<HashMap<&str, Expr>>();

    if let Some(root) = Expr::Var("root").eval(&mut monkeys.clone()) { println!("{root}") }

    monkeys.remove("humn");
    Expr::Var("root").eval(&mut monkeys);
    if let Expr::Op(_, box lhs, box rhs) = Expr::Var("root").build_tree(&monkeys)
    {
        let humn = match (lhs, rhs)
        {
            (Expr::Int(k), rhs) => solve(k, rhs),
            (lhs, Expr::Int(k)) => solve(k, lhs),
            _                   => None
        };

        if let Some(humn) = humn { println!("{humn}") }
    }
}

fn solve(acc : i64, expr : Expr) -> Option<i64>
{
    match expr
    {
        Expr::Op(Op::Add, lhs, box Expr::Int(l)) => solve(acc - l, *lhs),
        Expr::Op(Op::Add, box Expr::Int(k), rhs) => solve(acc - k, *rhs),

        Expr::Op(Op::Sub, lhs, box Expr::Int(l)) => solve(acc + l, *lhs),
        Expr::Op(Op::Sub, box Expr::Int(k), rhs) => solve(k - acc, *rhs),

        Expr::Op(Op::Mul, lhs, box Expr::Int(l)) => solve(acc / l, *lhs),
        Expr::Op(Op::Mul, box Expr::Int(k), rhs) => solve(acc / k, *rhs),

        Expr::Op(Op::Div, lhs, box Expr::Int(l)) => solve(acc * l, *lhs),
        Expr::Op(Op::Div, box Expr::Int(k), rhs) => solve(k / acc, *rhs),

        Expr::Var(_) => Some(acc),
        _            => None
    }
}

#[derive(Clone)]
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
    fn parse(s : &str) -> Option<Expr<'_>>
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
            Expr::Var(v) => match context.get(v).cloned()?
            {
                Expr::Int(k) => Some(k),
                expr         =>
                {
                    let k = expr.eval(context)?;
                    context.insert(v, Expr::Int(k));
                    Some(k)
                }
            },
            Expr::Op(op, lhs, rhs) =>
            {
                let lhs = lhs.eval(context);
                let rhs = rhs.eval(context);
                Some(op.eval(lhs?, rhs?))
            }
        }
    }

    fn build_tree(&self, context : &HashMap<&'a str, Expr<'a>>) -> Expr<'a>
    {
        match self
        {
            Expr::Int(k)           => Expr::Int(*k),
            Expr::Var(v)           => context.get(v).cloned()
                                             .map(|expr| expr.build_tree(context))
                                             .unwrap_or(Expr::Var(v)),
            Expr::Op(op, lhs, rhs) => Expr::Op(*op, Box::new(lhs.build_tree(context)),
                                                    Box::new(rhs.build_tree(context)))
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
