use std::collections::{ HashMap, VecDeque };

#[derive(Copy, Clone)]
pub enum Inst
{
    Set(u8, Value),
    Add(u8, Value),
    Sub(u8, Value),
    Mul(u8, Value),
    Mod(u8, Value),
    Jgz(Value, Value),
    Jnz(Value, Value),
    Snd(Value),
    Rcv(u8)
}

#[derive(Copy, Clone)]
pub enum Value
{
    Lit(i64),
    Reg(u8)
}

impl Inst
{
    pub fn parse(s : &str) -> Inst
    {
        match s.split_whitespace().collect::<Vec<&str>>()[..]
        {
            ["set", a, b] => Inst::Set(a.bytes().next().unwrap(), Value::parse(b)),
            ["add", a, b] => Inst::Add(a.bytes().next().unwrap(), Value::parse(b)),
            ["sub", a, b] => Inst::Sub(a.bytes().next().unwrap(), Value::parse(b)),
            ["mul", a, b] => Inst::Mul(a.bytes().next().unwrap(), Value::parse(b)),
            ["mod", a, b] => Inst::Mod(a.bytes().next().unwrap(), Value::parse(b)),
            ["jgz", a, b] => Inst::Jgz(Value::parse(a), Value::parse(b)),
            ["jnz", a, b] => Inst::Jnz(Value::parse(a), Value::parse(b)),
            ["snd", a]    => Inst::Snd(Value::parse(a)),
            ["rcv", a]    => Inst::Rcv(a.bytes().next().unwrap()),
            _             => unreachable!()
        }
    }

    pub fn prog_index(pc : i64, prog : &[Inst]) -> Option<&Inst>
    {
        pc.try_into().ok().and_then(|i : usize| prog.get(i))
    }

    pub fn step(self, pc : &mut i64, regs : &mut HashMap<u8, i64>)
    {
        match self
        {
            Inst::Set(a, b) => { let k = b.deref(regs);  regs.insert(a, k); },
            Inst::Add(a, b) => { let k = b.deref(regs); *regs.entry(a).or_insert(0) += k },
            Inst::Sub(a, b) => { let k = b.deref(regs); *regs.entry(a).or_insert(0) -= k },
            Inst::Mul(a, b) => { let k = b.deref(regs); *regs.entry(a).or_insert(0) *= k },
            Inst::Mod(a, b) => { let k = b.deref(regs); *regs.entry(a).or_insert(0) %= k },
            Inst::Jgz(a, b) => { let k = a.deref(regs); *pc += if k  > 0 { b.deref(regs) } else { 1 }},
            Inst::Jnz(a, b) => { let k = a.deref(regs); *pc += if k != 0 { b.deref(regs) } else { 1 }},
            _               => ()
        }

        if !self.is_jump() { *pc += 1 }
    }

    pub fn step_with_io(self, pc : &mut i64, regs : &mut HashMap<u8, i64>, input : &mut VecDeque<i64>, output : &mut VecDeque<i64>) -> bool
    {
        match self
        {
            Inst::Snd(a) =>
            {
                let k = a.deref(regs);
                output.push_back(k)
            }
            Inst::Rcv(a) =>
            {
                match input.pop_front()
                {
                    None    => return false,
                    Some(k) => { regs.insert(a, k); }
                }
            }
            _ => ()
        }

        self.step(pc, regs);
        true
    }

    fn is_jump(self) -> bool
    {
        matches!(self, Inst::Jgz(_, _) | Inst::Jnz(_, _))
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

    pub fn deref(self, regs : &HashMap<u8, i64>) -> i64
    {
        match self
        {
            Value::Lit(k) => k,
            Value::Reg(r) => regs.get(&r).copied().unwrap_or(0)
        }
    }
}
