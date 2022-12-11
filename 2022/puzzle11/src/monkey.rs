use std::{ collections::VecDeque, rc::Rc };

pub type WorryLevel = u64;

#[derive(Clone)]
pub struct Monkey
{
    pub items: VecDeque<WorryLevel>,
    op_fn:     Rc<dyn Fn(WorryLevel) -> WorryLevel>,
    test:      (WorryLevel, usize, usize)
}

impl Monkey
{
    pub fn op(&self, k : WorryLevel) -> WorryLevel
    {
        (self.op_fn)(k)
    }

    pub fn throw_to(&self, k : WorryLevel) -> usize
    {
        let (factor, if_t, if_f) = self.test;
        if k % factor == 0 { if_t } else { if_f }
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
        let op_fn     = Monkey::parse_op_fn(lines.next()?)?;
        let test      = Monkey::parse_test(lines)?;

        Some(Monkey { items, op_fn, test })
    }

    fn parse_op_fn(line : &str) -> Option<Rc<impl Fn(WorryLevel) -> WorryLevel>>
    {
        enum Op { Add, Mul }

        let mut words = line.strip_prefix("  Operation: new = ")?.split_whitespace();
        let arg_l     = words.next()?.parse::<WorryLevel>().ok();
        let op        = match words.next()? { "+" => Some(Op::Add), "*" => Some(Op::Mul), _ => None }?;
        let arg_r     = words.next()?.parse::<WorryLevel>().ok();

        Some(Rc::new(move |old| match op
        {
            Op::Add => arg_l.unwrap_or(old) + arg_r.unwrap_or(old),
            Op::Mul => arg_l.unwrap_or(old) * arg_r.unwrap_or(old)
        }))
    }

    fn parse_test<'a>(mut lines : impl Iterator<Item = &'a str>) -> Option<(WorryLevel, usize, usize)>
    {
        let factor = lines.next()?.strip_prefix("  Test: divisible by ")?.parse().ok()?;
        let if_t   = lines.next()?.strip_prefix("    If true: throw to monkey ")?.parse().ok()?;
        let if_f   = lines.next()?.strip_prefix("    If false: throw to monkey ")?.parse().ok()?;
        Some((factor, if_t, if_f))
    }
}
