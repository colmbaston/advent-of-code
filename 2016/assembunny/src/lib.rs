pub enum Inst
{
    Cpy(Value, Reg),
    Inc(Reg),
    Dec(Reg),
    Jnz(Value, i32)
}

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
            ["cpy", v, r] => Inst::Cpy(Value::parse(v), Reg::parse(r)),
            ["inc", r]    => Inst::Inc(Reg::parse(r)),
            ["dec", r]    => Inst::Dec(Reg::parse(r)),
            ["jnz", v, i] => Inst::Jnz(Value::parse(v), i.parse().unwrap()),
            _             => unreachable!()
        }
    }

    pub fn step(&self, pc : &mut i32, regs : &mut [i32 ; 4])
    {
        match self
        {
            Inst::Cpy(v, r) => { regs[*r as usize]  = v.deref(regs);     *pc += 1 },
            Inst::Inc(r)    => { regs[*r as usize] += 1;                 *pc += 1 },
            Inst::Dec(r)    => { regs[*r as usize] -= 1;                 *pc += 1 },
            Inst::Jnz(v, i) => if v.deref(regs) != 0 { *pc += i } else { *pc += 1 },
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
