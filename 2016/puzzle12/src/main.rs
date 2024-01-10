use assembunny::{ Inst, Reg };

fn main()
{
    let mut prog = include_str!("../input.txt").lines().map(Inst::parse).collect::<Vec<Inst>>();

    for mut regs in [[0, 0, 0, 0], [0, 0, 1, 0]]
    {
        let mut pc = 0;
        while let Some(&inst) = pc.try_into().ok().and_then(|i : usize| prog.get(i))
        {
            inst.step(&mut pc, &mut regs, &mut prog);
        }
        println!("{}", regs[Reg::A as usize]);
    }
}
