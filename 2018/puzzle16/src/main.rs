use std::collections::HashSet;

fn main()
{
    let (samples, program) = parse(include_str!("../input.txt"));

    // to begin with, list every opcode as a possibility for each value
    let mut possibilities = vec![OpCode::iter_all().collect::<HashSet<_>>() ; 16];

    // for each sample, try every opcode and eliminate the
    // possibilities whose results don't match the sample
    let mut match_three = 0;
    for Sample { before, instr, after } in samples.iter()
    {
        let mut matches = 0;
        let possible    = &mut possibilities[instr[0]];

        for op in OpCode::iter_all()
        {
            let mut before = *before;
            run_instruction(op, instr, &mut before);

            if &before == after
            {
                matches += 1
            }
            else
            {
                possible.remove(&op);
            }
        }

        if matches >= 3 { match_three += 1 };
    }

    // part 1: how many samples behaved like three or more opcodes?
    println!("{}", match_three);

    // if there is only one possibility left for a value, eliminate
    // it at as a possibility for the rest of the values
    let mut opcodes : [OpCode ; 16] = [OpCode::Addr ; 16];
    for _ in 0 .. 16
    {
        match possibilities.iter().enumerate().find(|(_, s)| s.len() == 1)
        {
            None         => panic!("could not discriminate all opcodes"),
            Some((i, s)) =>
            {
                let op = *s.iter().next().unwrap();
                opcodes[i] = op;
                for s in possibilities.iter_mut() { s.remove(&op); }
            }
        }
    }

    // part 2: run the program and print the value at register 0
    let mut regs = [0, 0, 0, 0];
    for instr in program.iter()
    {
        run_instruction(opcodes[instr[0]], instr, &mut regs);
    }
    println!("{}", regs[0]);
}

type Registers   = [usize ; 4];
type Instruction = [usize ; 4];

struct Sample
{
    before: Registers,
    instr:  Instruction,
    after:  Registers
}

fn parse(s : &str) -> (Vec<Sample>, Vec<Instruction>)
{
    fn parse_digits(s : &str) -> (usize, &str)
    {
        let (digits, s) = s.split_at(s.find(|c : char| !c.is_ascii_digit()).unwrap_or(s.len()));
        (digits.parse().unwrap(), s)
    }

    let mut samples  = Vec::new();
    let mut program  = Vec::new();
    let mut sections = s.split("\n\n\n\n");

    for s in sections.next().unwrap().split("\n\n")
    {
        let (b0, s) = parse_digits(&s[9..]);
        let (b1, s) = parse_digits(&s[2..]);
        let (b2, s) = parse_digits(&s[2..]);
        let (b3, s) = parse_digits(&s[2..]);
        let (i0, s) = parse_digits(&s[2..]);
        let (i1, s) = parse_digits(&s[1..]);
        let (i2, s) = parse_digits(&s[1..]);
        let (i3, s) = parse_digits(&s[1..]);
        let (a0, s) = parse_digits(&s[10..]);
        let (a1, s) = parse_digits(&s[2..]);
        let (a2, s) = parse_digits(&s[2..]);
        let (a3, _) = parse_digits(&s[2..]);

        samples.push(Sample
        {
            before: [b0, b1, b2, b3],
            instr:  [i0, i1, i2, i3],
            after:  [a0, a1, a2, a3]
        });
    }

    for s in sections.next().unwrap().lines()
    {
        let (i0, s) = parse_digits(&s[0..]);
        let (i1, s) = parse_digits(&s[1..]);
        let (i2, s) = parse_digits(&s[1..]);
        let (i3, _) = parse_digits(&s[1..]);

        program.push([i0, i1, i2, i3]);
    }

    (samples, program)
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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

impl OpCode
{
    fn iter_all() -> impl Iterator<Item = OpCode>
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

fn run_instruction(op : OpCode, &[_, a, b, c] : &Instruction, regs : &mut Registers)
{
    match op
    {
        OpCode::Addr => regs[c] = regs[a] + regs[b],
        OpCode::Addi => regs[c] = regs[a] + b,
        OpCode::Mulr => regs[c] = regs[a] * regs[b],
        OpCode::Muli => regs[c] = regs[a] * b,
        OpCode::Banr => regs[c] = regs[a] & regs[b],
        OpCode::Bani => regs[c] = regs[a] & b,
        OpCode::Borr => regs[c] = regs[a] | regs[b],
        OpCode::Bori => regs[c] = regs[a] | b,
        OpCode::Setr => regs[c] = regs[a],
        OpCode::Seti => regs[c] = a,
        OpCode::Gtir => regs[c] = (a       >  regs[b]) as usize,
        OpCode::Gtri => regs[c] = (regs[a] >  b)       as usize,
        OpCode::Gtrr => regs[c] = (regs[a] >  regs[b]) as usize,
        OpCode::Eqir => regs[c] = (a       == regs[b]) as usize,
        OpCode::Eqri => regs[c] = (regs[a] == b)       as usize,
        OpCode::Eqrr => regs[c] = (regs[a] == regs[b]) as usize
    }
}
