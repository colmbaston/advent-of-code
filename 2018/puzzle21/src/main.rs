use std::collections::HashSet;

fn main()
{
    let (pc_reg, program) = parse(include_str!("../input.txt"));

    // these values were found by reverse-engineering my inut
    // program, so they likely won't work for a different input
    const INSTRUCTION : usize = 28;
    const REGISTER    : usize =  5;

    // part 1: dump the value inside REGISTER after executing
    // the instruction at index INSTRUCTION for the first time
    let mut regs = [0 ; 6];
    while let Some(instr) = program.get(regs[pc_reg])
    {
        step(pc_reg, instr, &mut regs);
        if regs[pc_reg] == INSTRUCTION
        {
            println!("{}", regs[REGISTER]);
            break
        }
    }

    // part 2: dump the final value inside REGISTER after executing
    // the instruction at index INSTRUCTION before the value repeats
    let mut regs    = [0 ; 6];
    let mut last    = 0;
    let mut visited = HashSet::new();
    while let Some(instr) = program.get(regs[pc_reg])
    {
        step(pc_reg, instr, &mut regs);
        if regs[pc_reg] == INSTRUCTION
        {
            let value = regs[REGISTER];
            if !visited.insert(value)
            {
                break
            }
            last = value;
        }
    }
    println!("{}", last)
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

fn step(pc_reg : usize, &Instruction { op, a, b, c } : &Instruction, regs : &mut Registers)
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
