use std::collections::HashMap;

fn main()
{
    let input = include_str!("../input.txt").lines().collect::<Vec<_>>();

    let mut ops = HashMap::new();
    ops.insert('+', (Op::Add, 0));
    ops.insert('*', (Op::Mul, 0));
    println!("{}", input.iter().filter_map(|l| Expr::parse(l, &ops).map(|e| e.eval())).sum::<u64>());
    ops.insert('+', (Op::Add, 1));
    println!("{}", input.iter().filter_map(|l| Expr::parse(l, &ops).map(|e| e.eval())).sum::<u64>());
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

type OpTable = HashMap<char, (Op, u8)>;

impl Expr
{
    fn parse(s : &str, ops : &OpTable) -> Option<Expr>
    {
        let (x, s) = Expr::parse_ops(s.trim_start(), ops, 0)?;
        if s.trim_start().is_empty() { Some(x) } else { None }
    }

    fn parse_ops<'a>(s : &'a str, ops : &OpTable, prec : u8) -> Option<(Expr, &'a str)>
    {
        let (mut x, mut s) = Expr::parse_atom(s, ops)?;

        loop
        {
            s = s.trim_start();
            match s.chars().next().and_then(|c| ops.get(&c))
            {
                None          => break,
                Some((op, p)) => if *p < prec
                {
                    break
                }
                else
                {
                    let (y, t) = Expr::parse_ops(&s[1..].trim_start(), ops, *p + 1)?;
                    x = Expr::Op(op.clone(), Box::new(x), Box::new(y));
                    s = t;
                }
            }
        }

        Some((x, s))
    }

    fn parse_atom<'a>(s : &'a str, ops : &OpTable) -> Option<(Expr, &'a str)>
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
                let (x, s) = Expr::parse_ops(s.trim_start(), ops, 0)?;
                s.trim_start().strip_prefix(')').map(|s| (x, s))
            }
        }
    }
}
