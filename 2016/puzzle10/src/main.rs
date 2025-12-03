use std::collections::HashMap;

fn main()
{
    let mut insts = include_str!("../input.txt").lines().map(Inst::parse).collect::<Vec<Inst>>();
    insts.sort_unstable();
    let (init, insts) = insts.split_at(insts.iter().position(|i| matches!(i, Inst::Bot(_, _, _))).unwrap());

    let mut queue   = Vec::new();
    let mut bots    = HashMap::new();
    let mut outputs = HashMap::new();
    for inst in init.iter()
    {
        if let &Inst::Value(v, b) = inst
        {
            let bot = bots.entry(b).or_insert(Bot::Zero);
            bot.insert(v);
            if matches!(bot, Bot::Two(_, _)) { queue.push(b) }
        }
    }

    while let Some(bot) = queue.pop()
    {
        if let Some(&Bot::Two(low, high)) = bots.get(&bot)
        && let Inst::Bot(_, low_dest, high_dest) = insts[insts.binary_search_by_key(&bot, |i| i.bot()).unwrap()]
        {
            for (v, d) in [(low, low_dest), (high, high_dest)]
            {
                match d
                {
                    Dest::Output(o) => { outputs.insert(o, v); },
                    Dest::Bot(b)    =>
                    {
                        let bot = bots.entry(b).or_insert(Bot::Zero);
                        bot.insert(v);
                        if matches!(bot, Bot::Two(_, _)) { queue.push(b) }
                    }
                }
            }
        }
    }

    println!("{}", bots.iter().find(|(_, b)| matches!(b, Bot::Two(17, 61))).unwrap().0);
    println!("{}", outputs[&0] as u32 * outputs[&1] as u32 * outputs[&2] as u32);
}

enum Bot
{
    Zero,
    One(u8),
    Two(u8, u8)
}

impl Bot
{
    fn insert(&mut self, a : u8)
    {
        match self
        {
            Bot::Zero      => *self = Bot::One(a),
            Bot::One(b)    => *self = Bot::Two(a.min(*b), a.max(*b)),
            Bot::Two(_, _) => unreachable!()
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Inst
{
    Value(u8, u8),
    Bot(u8, Dest, Dest)
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Dest
{
    Bot(u8),
    Output(u8)
}

impl Inst
{
    fn parse(s : &str) -> Inst
    {
        if let Some(s) = s.strip_prefix("value ")
        {
            let (a, b) = s.split_once(" goes to bot ").unwrap();
            Inst::Value(a.parse().unwrap(), b.parse().unwrap())
        }
        else if let Some(s) = s.strip_prefix("bot ")
        {
            let (a, b) = s.split_once(" gives low to ").unwrap();
            let (b, c) = b.split_once(" and high to ").unwrap();
            Inst::Bot(a.parse().unwrap(), Dest::parse(b), Dest::parse(c))
        }
        else
        {
            unreachable!()
        }
    }

    fn bot(&self) -> u8
    {
        match self
        {
            Inst::Value(b, _)  => *b,
            Inst::Bot(b, _, _) => *b
        }
    }
}

impl Dest
{
    fn parse(s : &str) -> Dest
    {
        let (a, b) = s.split_once(' ').unwrap();

        match a
        {
            "bot"    => Dest::Bot(b.parse().unwrap()),
            "output" => Dest::Output(b.parse().unwrap()),
            _        => unreachable!()
        }
    }
}
