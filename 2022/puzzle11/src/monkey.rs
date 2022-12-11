use std::collections::VecDeque;

pub struct Monkey
{
    pub items: VecDeque<usize>,
    fn_op:     Box<dyn Fn(usize) -> usize>,
    fn_test:   Box<dyn Fn(usize) -> bool>,
    test_pos:  usize,
    test_neg:  usize
}

impl Monkey
{
    pub fn op(&self, k : usize) -> usize
    {
        (self.fn_op)(k)
    }

    pub fn test(&self, k : usize) -> bool
    {
        (self.fn_test)(k)
    }

    pub fn throw_to(&self, k : usize) -> usize
    {
        if self.test(k) { self.test_pos } else { self.test_neg }
    }

    pub fn parse(s : &str) -> Option<Monkey>
    {
        let mut lines = s.lines().skip(1);

        let items = lines.next()?
                         .strip_prefix("  Starting items: ")?
                         .split(", ")
                         .map(|w| w.parse::<usize>().ok())
                         .collect::<Option<VecDeque<usize>>>()?;

        let fn_op    = Monkey::parse_op(  lines.next()?.strip_prefix("  Operation: new = ")?)?;
        let fn_test  = Monkey::parse_test(lines.next()?.strip_prefix("  Test: "           )?)?;
        let test_pos = lines.next()?.strip_prefix("    If true: throw to monkey " )?.parse::<usize>().ok()?;
        let test_neg = lines.next()?.strip_prefix("    If false: throw to monkey ")?.parse::<usize>().ok()?;

        Some(Monkey { items, fn_op, fn_test, test_pos, test_neg })
    }

    fn parse_op(s : &str) -> Option<Box<impl Fn(usize) -> usize>>
    {
        enum Op { Add, Mul }

        let mut words = s.split_whitespace();
        let arg_l     = words.next()?.parse::<usize>().ok();
        let op        = match words.next()? { "+" => Some(Op::Add), "*" => Some(Op::Mul), _ => None }?;
        let arg_r     = words.next()?.parse::<usize>().ok();

        Some(Box::new(move |old| match op
        {
            Op::Add => arg_l.unwrap_or(old) + arg_r.unwrap_or(old),
            Op::Mul => arg_l.unwrap_or(old) * arg_r.unwrap_or(old)
        }))
    }

    fn parse_test(s : &str) -> Option<Box<impl Fn(usize) -> bool>>
    {
        s.strip_prefix("divisible by ")
         .and_then(|w| w.parse::<usize>().ok().map(|k| Box::new(move |v| v % k == 0)))
    }
}
