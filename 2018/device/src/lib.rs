#![feature(iter_next_chunk)]

pub struct Inst
{
    pub op: OpCode,
    pub a:  usize,
    pub b:  usize,
    pub c:  usize
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum OpCode
{
    Addr, Addi,
    Mulr, Muli,
    Banr, Bani,
    Borr, Bori,
    Setr, Seti,
    Gtir, Gtri, Gtrr,
    Eqir, Eqri, Eqrr
}

impl Inst
{
    pub fn parse(s : &str) -> Inst
    {
        let mut words = s.split_whitespace();

        let op = match words.next().unwrap()
        {
            "addr" => OpCode::Addr,
            "addi" => OpCode::Addi,
            "mulr" => OpCode::Mulr,
            "muli" => OpCode::Muli,
            "banr" => OpCode::Banr,
            "bani" => OpCode::Bani,
            "borr" => OpCode::Borr,
            "bori" => OpCode::Bori,
            "setr" => OpCode::Setr,
            "seti" => OpCode::Seti,
            "gtir" => OpCode::Gtir,
            "gtri" => OpCode::Gtri,
            "gtrr" => OpCode::Gtrr,
            "eqir" => OpCode::Eqir,
            "eqri" => OpCode::Eqri,
            "eqrr" => OpCode::Eqrr,
            _      => unreachable!()
        };

        let [a, b, c] = words.map(|k| k.parse().unwrap())
                             .next_chunk().unwrap();

        Inst { op, a, b, c }
    }

    pub fn step(&self, regs : &mut [usize])
    {
        match self.op
        {
            OpCode::Addr => regs[self.c] =  regs[self.a] +  regs[self.b],
            OpCode::Addi => regs[self.c] =  regs[self.a] +       self.b,
            OpCode::Mulr => regs[self.c] =  regs[self.a] *  regs[self.b],
            OpCode::Muli => regs[self.c] =  regs[self.a] *       self.b,
            OpCode::Banr => regs[self.c] =  regs[self.a] &  regs[self.b],
            OpCode::Bani => regs[self.c] =  regs[self.a] &       self.b,
            OpCode::Borr => regs[self.c] =  regs[self.a] |  regs[self.b],
            OpCode::Bori => regs[self.c] =  regs[self.a] |       self.b,
            OpCode::Setr => regs[self.c] =  regs[self.a],
            OpCode::Seti => regs[self.c] =       self.a,
            OpCode::Gtir => regs[self.c] = (     self.a  >  regs[self.b]) as usize,
            OpCode::Gtri => regs[self.c] = (regs[self.a] >       self.b ) as usize,
            OpCode::Gtrr => regs[self.c] = (regs[self.a] >  regs[self.b]) as usize,
            OpCode::Eqir => regs[self.c] = (     self.a  == regs[self.b]) as usize,
            OpCode::Eqri => regs[self.c] = (regs[self.a] ==      self.b ) as usize,
            OpCode::Eqrr => regs[self.c] = (regs[self.a] == regs[self.b]) as usize
        }
    }

    pub fn step_with_pc(&self, pc : usize, regs : &mut [usize])
    {
        self.step(regs);
        regs[pc] += 1;
    }
}

impl OpCode
{
    pub fn enumerate() -> impl Iterator<Item = OpCode>
    {
        [OpCode::Addr, OpCode::Addi,
         OpCode::Mulr, OpCode::Muli,
         OpCode::Banr, OpCode::Bani,
         OpCode::Borr, OpCode::Bori,
         OpCode::Setr, OpCode::Seti,
         OpCode::Gtir, OpCode::Gtri, OpCode::Gtrr,
         OpCode::Eqir, OpCode::Eqri, OpCode::Eqrr].into_iter()
    }
}
