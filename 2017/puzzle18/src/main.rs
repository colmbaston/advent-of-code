use tablet::{ Inst, Value };
use std::collections::{ VecDeque, HashMap };

fn main()
{
    let mut pc_a    = 0;
    let mut pc_b    = 0;
    let mut regs_a  = HashMap::new();
    let mut regs_b  = HashMap::new();
    let mut queue_a = VecDeque::new();
    let mut queue_b = VecDeque::new();
    let prog        = include_str!("../input.txt").lines().map(Inst::parse).collect::<Vec<Inst>>();

    let mut last_snd = 0;
    while let Some(&inst) = Inst::prog_index(pc_a, &prog)
    {
        match inst
        {
            Inst::Snd(a) => { last_snd = a.deref(&regs_a); pc_a += 1 },
            Inst::Rcv(a) => if Value::Reg(a).deref(&regs_a) != 0 { println!("{last_snd}"); break },
            _            => inst.step(&mut pc_a, &mut regs_a)
        }
    }

    pc_a = 0;
    regs_a.clear();
    regs_a.insert(b'p', 0);
    regs_b.insert(b'p', 1);
    let mut count      = 0;
    let mut progress_a = true;
    let mut progress_b = true;
    while progress_a || progress_b
    {
        progress_a = Inst::prog_index(pc_a, &prog).map(|inst|
        {
            inst.step_with_io(&mut pc_a, &mut regs_a, &mut queue_a, &mut queue_b)
        })
        .unwrap_or(false);

        progress_b = Inst::prog_index(pc_b, &prog).map(|inst|
        {
            if matches!(inst, Inst::Snd(_)) { count += 1 }
            inst.step_with_io(&mut pc_b, &mut regs_b, &mut queue_b, &mut queue_a)
        })
        .unwrap_or(false)
    }
    println!("{count}");
}
