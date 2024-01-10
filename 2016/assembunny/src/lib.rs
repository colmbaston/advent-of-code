#[derive(Copy, Clone)]
pub enum Inst
{
    Cpy(Value, Value),
    Inc(Value),
    Dec(Value),
    Jnz(Value, Value),
    Tgl(Value)
}

#[derive(Copy, Clone)]
pub enum Value
{
    Lit(i32),
    Reg(Reg)
}

#[derive(Copy, Clone)]
pub enum Reg { A, B, C, D }

impl Inst
{
    pub fn parse(s : &str) -> Inst
    {
        match s.split_whitespace().collect::<Vec<&str>>()[..]
        {
            ["cpy", a, b] => Inst::Cpy(Value::parse(a), Value::parse(b)),
            ["inc", a]    => Inst::Inc(Value::parse(a)),
            ["dec", a]    => Inst::Dec(Value::parse(a)),
            ["jnz", a, b] => Inst::Jnz(Value::parse(a), Value::parse(b)),
            ["tgl", a]    => Inst::Tgl(Value::parse(a)),
            _             => unreachable!()
        }
    }

    pub fn step(&self, pc : &mut i32, regs : &mut [i32 ; 4], prog : &mut [Inst])
    {
        match self
        {
            Inst::Cpy(a, Value::Reg(b)) => { regs[*b as usize]  = a.deref(regs); *pc += 1 },
            Inst::Inc(Value::Reg(a))    => { regs[*a as usize] += 1;             *pc += 1 },
            Inst::Dec(Value::Reg(a))    => { regs[*a as usize] -= 1;             *pc += 1 },
            Inst::Jnz(a, b)             => *pc += if a.deref(regs) != 0 { b.deref(regs) } else { 1 },
            Inst::Tgl(a)                =>
            {
                (*pc + a.deref(regs)).try_into().ok()
                                     .and_then(|i : usize| prog.get_mut(i))
                                     .map(Inst::tgl);
                *pc += 1;
            },
            _ => ()
        }
    }

    fn tgl(&mut self)
    {
        *self = match *self
        {
            Inst::Cpy(a, b) => Inst::Jnz(a, b),
            Inst::Inc(a)    => Inst::Dec(a),
            Inst::Dec(a)    => Inst::Inc(a),
            Inst::Jnz(a, b) => Inst::Cpy(a, b),
            Inst::Tgl(a)    => Inst::Inc(a)
        }
    }
}

impl Value
{
    fn parse(s : &str) -> Value
    {
        s.parse().map(Value::Lit)
         .unwrap_or_else(|_| Value::Reg(Reg::parse(s)))
    }

    fn deref(&self, regs : &[i32 ; 4]) -> i32
    {
        match self
        {
            Value::Lit(k) => *k,
            Value::Reg(r) => regs[*r as usize]
        }
    }
}

impl Reg
{
    fn parse(s : &str) -> Reg
    {
        match s
        {
            "a" => Reg::A,
            "b" => Reg::B,
            "c" => Reg::C,
            "d" => Reg::D,
            _   => unreachable!()
        }
    }
}
