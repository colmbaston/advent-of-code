use assembunny::Inst;
use std::collections::HashSet;

fn main()
{
    let prog = include_str!("../input.txt").lines()
                                           .map(Inst::parse)
                                           .collect::<Vec<Inst>>();

    let mut visited = HashSet::new();
    'outer: for a in 1 ..
    {
        let mut pc   = 0;
        let mut regs = [a, 0, 0, 0];
        let mut prog = prog.clone();

        visited.clear();
        for signal in (0 ..= 1).cycle()
        {
            if !visited.insert((pc, regs, prog.clone()))
            {
                println!("{a}");
                break 'outer
            }

            if Inst::out(&mut pc, &mut regs, &mut prog).filter(|&out| out == signal).is_none()
            {
                break
            }
        }
    }
}
