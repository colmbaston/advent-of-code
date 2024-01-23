use tablet::Inst;
use std::collections::HashMap;

fn main()
{
    let mut pc   = 0;
    let mut regs = HashMap::new();
    let prog     = include_str!("../input.txt").lines().map(Inst::parse).collect::<Vec<Inst>>();

    let mut count = 0;
    while let Some(&inst) = Inst::prog_index(pc, &prog)
    {
        if matches!(inst, Inst::Mul(_, _)) { count += 1 }
        inst.step(&mut pc, &mut regs)
    }
    println!("{count}");

    // algorithm found by reverse-engineering my input program, so the
    // solution is partially hard-coded and may not work for other inputs
    pc = 0;
    regs.clear();
    regs.insert(b'a', 1);
    while pc < 8
    {
        if let Some(&inst) = Inst::prog_index(pc, &prog)
        {
            inst.step(&mut pc, &mut regs)
        }
    }
    println!("{}", (regs[&b'b'] ..= regs[&b'c']).step_by(17).filter(|&k| is_composite(k)).count());
}

fn is_composite(k : i64) -> bool
{
    std::iter::once(2).chain((3 ..).step_by(2))
                      .take_while(|j| j*j <= k)
                      .any(|j| k % j == 0)
}
