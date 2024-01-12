use assembunny::{ Inst, Reg };

fn main()
{
    let prog = include_str!("../input.txt").lines().map(Inst::parse).collect::<Vec<Inst>>();

    for mut regs in [[7, 0, 0, 0], [12, 0, 0, 0]]
    {
        let mut pc   = 0;
        let mut prog = prog.clone();
        Inst::run(&mut pc, &mut regs, &mut prog);
        println!("{}", regs[Reg::A as usize]);
    }
}
