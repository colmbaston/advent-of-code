use std::collections::{ VecDeque, HashMap };

fn main()
{
    let mut pc_a    = 0;
    let mut pc_b    = 0;
    let mut regs_a  = HashMap::new();
    let mut regs_b  = HashMap::new();
    let mut queue_a = VecDeque::new();
    let mut queue_b = VecDeque::new();
    let prog        = include_str!("../input.txt").lines().map(Inst::parse).collect::<Vec<Inst>>();

    let mut last_snd = 0;
    while let Some(&inst) = Inst::prog_index(pc_a, &prog)
    {
        match inst
        {
            Inst::Snd(a) => { last_snd = a.deref(&regs_a); pc_a += 1 },
            Inst::Rcv(a) => if Value::Reg(a).deref(&regs_a) != 0 { println!("{last_snd}"); break },
            _            => { inst.step(&mut pc_a, &mut regs_a, &mut queue_a, &mut queue_b); }
        }
    }

    pc_a = 0;
    regs_a.clear();
    regs_a.insert(b'p', 0);
    regs_b.insert(b'p', 1);
    let mut count      = 0;
    let mut progress_a = true;
    let mut progress_b = true;
    while progress_a || progress_b
    {
        progress_a = Inst::prog_index(pc_a, &prog).map(|inst|
        {
            inst.step(&mut pc_a, &mut regs_a, &mut queue_a, &mut queue_b)
        })
        .unwrap_or(false);

        progress_b = Inst::prog_index(pc_b, &prog).map(|inst|
        {
            if matches!(inst, Inst::Snd(_)) { count += 1 }
            inst.step(&mut pc_b, &mut regs_b, &mut queue_b, &mut queue_a)
        })
        .unwrap_or(false)
    }
    println!("{count}");
}


#[derive(Copy, Clone)]
enum Inst
{
    Set(Reg, Value),
    Add(Reg, Value),
    Mul(Reg, Value),
    Mod(Reg, Value),
    Jgz(Value, Value),
    Snd(Value),
    Rcv(Reg)
}

type Reg = u8;

#[derive(Copy, Clone)]
enum Value
{
    Lit(i64),
    Reg(Reg)
}

type Regs  = HashMap<Reg, i64>;
type Queue = VecDeque<i64>;

impl Inst
{
    fn parse(s : &str) -> Inst
    {
        match s.split_whitespace().collect::<Vec<&str>>()[..]
        {
            ["set", a, b] => Inst::Set(a.bytes().next().unwrap(), Value::parse(b)),
            ["add", a, b] => Inst::Add(a.bytes().next().unwrap(), Value::parse(b)),
            ["mul", a, b] => Inst::Mul(a.bytes().next().unwrap(), Value::parse(b)),
            ["mod", a, b] => Inst::Mod(a.bytes().next().unwrap(), Value::parse(b)),
            ["jgz", a, b] => Inst::Jgz(Value::parse(a), Value::parse(b)),
            ["snd", a]    => Inst::Snd(Value::parse(a)),
            ["rcv", a]    => Inst::Rcv(a.bytes().next().unwrap()),
            _             => unreachable!()
        }
    }

    fn prog_index(pc : i64, prog : &[Inst]) -> Option<&Inst>
    {
        pc.try_into().ok().and_then(|i : usize| prog.get(i))
    }

    fn step(self, pc : &mut i64, regs : &mut Regs, input : &mut Queue, output : &mut Queue) -> bool
    {
        match self
        {
            Inst::Set(a, b) => { let k = b.deref(regs);  regs.insert(a, k); },
            Inst::Add(a, b) => { let k = b.deref(regs); *regs.entry(a).or_insert(0) += k },
            Inst::Mul(a, b) => { let k = b.deref(regs); *regs.entry(a).or_insert(0) *= k },
            Inst::Mod(a, b) => { let k = b.deref(regs); *regs.entry(a).or_insert(0) %= k },
            Inst::Jgz(a, b) => { let k = a.deref(regs); *pc += if k > 0 { b.deref(regs) } else { 1 }},
            Inst::Snd(a)    => { let k = a.deref(regs); output.push_back(k) },
            Inst::Rcv(a)    =>
            {
                match input.pop_front()
                {
                    None    => return false,
                    Some(k) => { regs.insert(a, k); }
                }
            }
        }

        if !matches!(self, Inst::Jgz(_, _)) { *pc += 1 }
        true
    }
}

impl Value
{
    fn parse(s : &str) -> Value
    {
        match s.parse::<i64>()
        {
            Ok(k)  => Value::Lit(k),
            Err(_) => Value::Reg(s.bytes().next().unwrap())
        }
    }

    fn deref(self, regs : &Regs) -> i64
    {
        match self
        {
            Value::Lit(k) => k,
            Value::Reg(r) => regs.get(&r).copied().unwrap_or(0)
        }
    }
}
