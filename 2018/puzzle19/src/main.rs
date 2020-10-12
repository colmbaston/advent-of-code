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
    ADDR, ADDI,
    MULR, MULI,
    BANR, BANI,
    BORR, BORI,
    SETR, SETI,
    GTIR, GTRI, GTRR,
    EQIR, EQRI, EQRR
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
        "addr" => OpCode::ADDR,
        "addi" => OpCode::ADDI,
        "mulr" => OpCode::MULR,
        "muli" => OpCode::MULI,
        "banr" => OpCode::BANR,
        "bani" => OpCode::BANI,
        "borr" => OpCode::BORR,
        "bori" => OpCode::BORI,
        "setr" => OpCode::SETR,
        "seti" => OpCode::SETI,
        "gtir" => OpCode::GTIR,
        "gtri" => OpCode::GTRI,
        "gtrr" => OpCode::GTRR,
        "eqir" => OpCode::EQIR,
        "eqri" => OpCode::EQRI,
        "eqrr" => OpCode::EQRR,
        _      => unreachable!()
    };

    let a = it.next().unwrap().parse().unwrap();
    let b = it.next().unwrap().parse().unwrap();
    let c = it.next().unwrap().parse().unwrap();

    Instruction { op, a, b, c }
}

fn run(pc_reg : usize, program : &[Instruction], regs : &mut Registers)
{
    while let Some(&Instruction { op, a, b, c }) = program.get(regs[pc_reg])
    {
        match op
        {
            OpCode::ADDR => regs[c] =  regs[a] +  regs[b],
            OpCode::ADDI => regs[c] =  regs[a] +       b,
            OpCode::MULR => regs[c] =  regs[a] *  regs[b],
            OpCode::MULI => regs[c] =  regs[a] *       b,
            OpCode::BANR => regs[c] =  regs[a] &  regs[b],
            OpCode::BANI => regs[c] =  regs[a] &       b,
            OpCode::BORR => regs[c] =  regs[a] |  regs[b],
            OpCode::BORI => regs[c] =  regs[a] |       b,
            OpCode::SETR => regs[c] =  regs[a],
            OpCode::SETI => regs[c] =       a,
            OpCode::GTIR => regs[c] = (     a  >  regs[b]) as usize,
            OpCode::GTRI => regs[c] = (regs[a] >       b ) as usize,
            OpCode::GTRR => regs[c] = (regs[a] >  regs[b]) as usize,
            OpCode::EQIR => regs[c] = (     a  == regs[b]) as usize,
            OpCode::EQRI => regs[c] = (regs[a] ==      b ) as usize,
            OpCode::EQRR => regs[c] = (regs[a] == regs[b]) as usize
        }

        regs[pc_reg] += 1;
    }
}
