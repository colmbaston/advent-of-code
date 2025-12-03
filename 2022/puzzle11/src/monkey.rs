use std::{ collections::VecDeque };

pub type WorryLevel = u64;

#[derive(Clone)]
pub struct Monkey
{
    pub items: VecDeque<WorryLevel>,
    op:        Op,
    test:      (WorryLevel, usize, usize)
}

#[derive(Clone)]
enum Op  { Add(Val, Val), Mul(Val, Val) }

#[derive(Clone)]
enum Val { Old, Const(u64) }

impl Monkey
{
    pub fn op(&self, k : WorryLevel) -> WorryLevel
    {
        self.op.apply(k)
    }

    pub fn throw_to(&self, k : WorryLevel) -> usize
    {
        let (factor, if_t, if_f) = self.test;
        if k.is_multiple_of(factor) { if_t } else { if_f }
    }

    pub fn product<'a>(monkeys : impl Iterator<Item = &'a Monkey>) -> WorryLevel
    {
        monkeys.map(|m| m.test.0).product()
    }

    pub fn parse(s : &str) -> Option<Monkey>
    {
        let mut lines = s.lines().skip(1);
        let items     = lines.next()?
                             .strip_prefix("  Starting items: ")?
                             .split(", ")
                             .map(|w| w.parse::<WorryLevel>().ok())
                             .collect::<Option<VecDeque<WorryLevel>>>()?;
        let op   = Monkey::parse_op(lines.next()?)?;
        let test = Monkey::parse_test(lines)?;

        Some(Monkey { items, op, test })
    }

    fn parse_op(line : &str) -> Option<Op>
    {
        let mut words = line.strip_prefix("  Operation: new = ")?.split_whitespace();
        let l  = words.next()?.parse::<WorryLevel>().map(Val::Const).unwrap_or(Val::Old);
        let op = words.next()?;
        let r  = words.next()?.parse::<WorryLevel>().map(Val::Const).unwrap_or(Val::Old);

        match op
        {
            "+" => Some(Op::Add(l, r)),
            "*" => Some(Op::Mul(l, r)),
            _   => None
        }
    }

    fn parse_test<'a>(mut lines : impl Iterator<Item = &'a str>) -> Option<(WorryLevel, usize, usize)>
    {
        let factor = lines.next()?.strip_prefix("  Test: divisible by ")?.parse().ok()?;
        let if_t   = lines.next()?.strip_prefix("    If true: throw to monkey ")?.parse().ok()?;
        let if_f   = lines.next()?.strip_prefix("    If false: throw to monkey ")?.parse().ok()?;
        Some((factor, if_t, if_f))
    }
}

impl Op
{
    fn apply(&self, k : u64) -> u64
    {
        match self
        {
            Op::Add(a, b) => a.apply(k) + b.apply(k),
            Op::Mul(a, b) => a.apply(k) * b.apply(k)
        }
    }
}

impl Val
{
    fn apply(&self, k : u64) -> u64
    {
        match self
        {
            Val::Old      => k,
            Val::Const(j) => *j
        }
    }
}
