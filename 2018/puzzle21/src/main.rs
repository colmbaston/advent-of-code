use device::Inst;
use std::collections::HashSet;

fn main()
{
    let (pc, prog) = include_str!("../input.txt").split_once('\n').unwrap();
    let  pc        = pc.strip_prefix("#ip ").unwrap().parse::<usize>().unwrap();
    let      prog  = prog.lines().map(Inst::parse).collect::<Vec<Inst>>();

    // these values were found by reverse-engineering my inut
    // program, so they likely won't work for a different input
    const INST : usize = 28;
    const REG  : usize =  5;

    // part 1: dump the value inside REG after executing
    // the instruction at index INST for the first time
    let mut regs = [0 ; 6];
    while let Some(inst) = prog.get(regs[pc])
    {
        inst.step_with_pc(pc, &mut regs);
        if regs[pc] == INST
        {
            println!("{}", regs[REG]);
            break
        }
    }

    // part 2: dump the final value inside REG after executing
    // the instruction at index INST before the value repeats
    regs.fill(0);
    let mut last    = 0;
    let mut visited = HashSet::new();
    while let Some(inst) = prog.get(regs[pc])
    {
        inst.step_with_pc(pc, &mut regs);
        if regs[pc] == INST
        {
            let value = regs[REG];
            if !visited.insert(value)
            {
                println!("{last}");
                break
            }
            last = value;
        }
    }
}
