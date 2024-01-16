#![feature(iter_next_chunk)]
use std::{ hash::Hash, collections::HashMap };

fn main()
{
    let mut max  = 0;
    let mut regs = HashMap::new();
    for inst in include_str!("../input.txt").lines().map(Inst::parse)
    {
        if let Some(k) = inst.run(&mut regs) { max = max.max(k) }
    }
    println!("{}", regs.values().max().unwrap());
    println!("{max}");
}

struct Inst<T>
{
    reg:    T,
    offset: i32,
    cond:   Cond<T>
}

struct Cond<T>
{
    lhs: T,
    op:  Op,
    rhs: i32
}

#[derive(Copy, Clone)]
enum Op { Eq, Neq, Lt, Lte, Gt, Gte }

impl Inst<&str>
{
    fn parse(s : &str) -> Inst<&str>
    {
        let (a, b)    = s.split_once(" if ").unwrap();
        let [c, d, e] = a.split_whitespace().next_chunk().unwrap();
        let offset    = e.parse().unwrap();
        Inst { reg: c, offset: if d == "inc" { offset } else { -offset }, cond: Cond::parse(b) }
    }
}

impl<T : Eq + Hash + Copy> Inst<T>
{
    fn run(&self, regs : &mut HashMap<T, i32>) -> Option<i32>
    {
        self.cond.apply(regs).then(||
        {
            let k = regs.entry(self.reg).or_insert(0);
            *k += self.offset;
            *k
        })
    }
}

impl Cond<&str>
{
    fn parse(s : &str) -> Cond<&str>
    {
        let [a, b, c] = s.split_whitespace().next_chunk().unwrap();
        Cond { lhs: a, op: Op::parse(b), rhs: c.parse().unwrap() }
    }
}

impl<T : Eq + Hash> Cond<T>
{
    fn apply(&self, regs : &HashMap<T, i32>) -> bool
    {
        self.op.apply(regs.get(&self.lhs).copied().unwrap_or(0), self.rhs)
    }
}

impl Op
{
    fn parse(s : &str) -> Op
    {
        match s
        {
            "==" => Op::Eq,
            "!=" => Op::Neq,
            "<"  => Op::Lt,
            "<=" => Op::Lte,
            ">"  => Op::Gt,
            ">=" => Op::Gte,
            _    => unreachable!()
        }
    }

    fn apply<T : Ord>(self, a : T, b : T) -> bool
    {
        match self
        {
            Op::Eq  => a == b,
            Op::Neq => a != b,
            Op::Lt  => a <  b,
            Op::Lte => a <= b,
            Op::Gt  => a >  b,
            Op::Gte => a >= b
        }
    }
}
