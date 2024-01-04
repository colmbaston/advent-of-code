fn main()
{
    let (pc_reg, program) = parse(include_str!("../input.txt"));

    // part 1: run the program and print the
    // value left in register 0 when it halts
    let mut regs = [0 ; 6];
    run(pc_reg, &program, &mut regs);
    println!("{}", regs[0]);

    // part 2: by reverse-engineering the program, I learned that it was
    // computing the sum of the factors of VALUE; since, in principle, another
    // input could implement a different function, I leave this result hard-coded
    const VALUE : usize = 10_551_326;
    println!("{}", (1 ..= VALUE).filter(|n| VALUE % n == 0).sum::<usize>());
}

type Registers = [usize ; 6];

struct Instruction
{
    op: OpCode,
    a:  usize,
    b:  usize,
    c:  usize
}

#[derive(Clone, Copy)]
enum OpCode
{
    Addr, Addi,
    Mulr, Muli,
    Banr, Bani,
    Borr, Bori,
    Setr, Seti,
    Gtir, Gtri, Gtrr,
    Eqir, Eqri, Eqrr
}

fn parse(s : &str) -> (usize, Vec<Instruction>)
{
    let mut it = s.lines();
    let pc_reg = it.next().unwrap()[4..].parse().unwrap();

    (pc_reg, it.map(parse_instruction).collect())
}

fn parse_instruction(s : &str) -> Instruction
{
    let mut it = s.split_whitespace();

    let op = match it.next().unwrap()
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

    let a = it.next().unwrap().parse().unwrap();
    let b = it.next().unwrap().parse().unwrap();
    let c = it.next().unwrap().parse().unwrap();

    Instruction { op, a, b, c }
}

fn run(pc_reg : usize, program : &[Instruction], regs : &mut Registers)
{
    while let Some(instr) = program.get(regs[pc_reg])
    {
        step(pc_reg, instr, regs)
    }
}

fn step(pc_reg : usize, &Instruction { op, a, b, c } : &Instruction, regs : &mut Registers)
{
    match op
    {
        OpCode::Addr => regs[c] =  regs[a] +  regs[b],
        OpCode::Addi => regs[c] =  regs[a] +       b,
        OpCode::Mulr => regs[c] =  regs[a] *  regs[b],
        OpCode::Muli => regs[c] =  regs[a] *       b,
        OpCode::Banr => regs[c] =  regs[a] &  regs[b],
        OpCode::Bani => regs[c] =  regs[a] &       b,
        OpCode::Borr => regs[c] =  regs[a] |  regs[b],
        OpCode::Bori => regs[c] =  regs[a] |       b,
        OpCode::Setr => regs[c] =  regs[a],
        OpCode::Seti => regs[c] =       a,
        OpCode::Gtir => regs[c] = (     a  >  regs[b]) as usize,
        OpCode::Gtri => regs[c] = (regs[a] >       b ) as usize,
        OpCode::Gtrr => regs[c] = (regs[a] >  regs[b]) as usize,
        OpCode::Eqir => regs[c] = (     a  == regs[b]) as usize,
        OpCode::Eqri => regs[c] = (regs[a] ==      b ) as usize,
        OpCode::Eqrr => regs[c] = (regs[a] == regs[b]) as usize
    }
    regs[pc_reg] += 1;
}
