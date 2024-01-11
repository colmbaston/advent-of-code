#![feature(iter_next_chunk)]
use device::{ OpCode, Inst };
use std::collections::{ HashSet, HashMap };

fn main()
{
    let (samples, prog) = include_str!("../input.txt").split_once("\n\n\n\n").unwrap();
    let  samples        = samples.split("\n\n").map(Sample::parse).collect::<Vec<Sample>>();
    let           prog  = prog.lines().map(Sample::parse_inst).collect::<Vec<[usize ; 4]>>();

    // to begin with, list every opcode as a possibility for each value
    let mut possible = vec![OpCode::enumerate().collect::<HashSet<OpCode>>() ; 16];

    // for each sample, try every opcode and eliminate the
    // possibilities whose results don't match the sample
    println!("{}", samples.iter().filter(|Sample { before, inst, after }|
    {
        let mut matches = 0;
        for op in OpCode::enumerate()
        {
            let [_, a, b, c] = *inst;
            let mut regs     = *before;
            (Inst { op, a, b, c }).step(&mut regs);

            if &regs == after
            {
                matches += 1
            }
            else
            {
                possible[inst[0]].remove(&op);
            }
        }
        matches >= 3
    })
    .count());

    // if there is only one possibility left for a value, eliminate
    // it at as a possibility for the rest of the values
    let mut ops = HashMap::new();
    for _ in 0 .. 16
    {
        while let Some((i, s)) = possible.iter().enumerate().find(|(_, s)| s.len() == 1)
        {
            let op = *s.iter().next().unwrap();
            possible.iter_mut().for_each(|s| { s.remove(&op); });
            ops.insert(i, op);
        }
    }

    // part 2: run the program and print the value at register 0
    let mut regs = [0 ; 4];
    for &[op, a, b, c] in prog.iter()
    {
        (Inst { op: ops[&op], a, b, c }).step(&mut regs);
    }
    println!("{}", regs[0]);
}

struct Sample
{
    before: [usize ; 4],
    inst:   [usize ; 4],
    after:  [usize ; 4]
}

impl Sample
{
    fn parse(s : &str) -> Sample
    {
        let (a, b) = s.split_once('\n').unwrap();
        let (b, c) = b.split_once('\n').unwrap();

        let before = Sample::parse_regs(a.strip_prefix("Before: ").unwrap());
        let inst   = Sample::parse_inst(b);
        let after  = Sample::parse_regs(c.strip_prefix("After:  ").unwrap());

        Sample { before, inst, after }
    }

    fn parse_regs(s : &str) -> [usize ; 4]
    {
        s.strip_prefix('[').unwrap()
         .strip_suffix(']').unwrap()
         .split(", ")
         .map(|k| k.parse().unwrap())
         .next_chunk().unwrap()
    }

    fn parse_inst(s : &str) -> [usize ; 4]
    {
        s.split_whitespace()
         .map(|k| k.parse().unwrap())
         .next_chunk().unwrap()
    }
}
