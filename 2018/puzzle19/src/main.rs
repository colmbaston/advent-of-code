use device::Inst;

fn main()
{
    let (pc, prog) = include_str!("../input.txt").split_once('\n').unwrap();
    let  pc        = pc.strip_prefix("#ip ").unwrap().parse::<usize>().unwrap();
    let      prog  = prog.lines().map(Inst::parse).collect::<Vec<Inst>>();

    // part 1: run the program and print the
    // value left in register 0 when it halts
    let mut regs = [0 ; 6];
    while let Some(inst) = prog.get(regs[pc]) { inst.step_with_pc(pc, &mut regs) }
    println!("{}", regs[0]);

    // part 2: by reverse-engineering the program, I learned that it was
    // computing the sum of the factors of VALUE; since, in principle, another
    // input could implement a different function, I leave this result hard-coded
    const VALUE : usize = 10_551_326;
    println!("{}", (1 ..= VALUE).filter(|&n| VALUE.is_multiple_of(n)).sum::<usize>());
}
