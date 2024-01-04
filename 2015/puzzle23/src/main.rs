fn main()
{
    let input = include_str!("../input.txt").lines().map(parse_inst).collect::<Vec<Inst>>();

    for regs in [[0, 0], [1, 0]].iter_mut()
    {
        let mut pc = 0;
        while let Some(i) = input.get(pc as usize)
        {
            run_inst(i, regs, &mut pc);
        }
        println!("{}", regs[1]);
    }
}

enum Inst
{
    Hlf(usize),
    Tpl(usize),
    Inc(usize),
    Jmp(       isize),
    Jie(usize, isize),
    Jio(usize, isize)
}

fn parse_inst(s : &str) -> Inst
{
    let parse_reg = |r : &str|
    {
        match r.strip_suffix(',').unwrap_or(r)
        {
            "a" => 0,
            "b" => 1,
            _   => unreachable!()
        }
    };

    match s.split_whitespace().collect::<Vec<_>>()[..]
    {
        ["hlf", r]    => Inst::Hlf(parse_reg(r)),
        ["tpl", r]    => Inst::Tpl(parse_reg(r)),
        ["inc", r]    => Inst::Inc(parse_reg(r)),
        ["jmp",    o] => Inst::Jmp(              o.parse().unwrap()),
        ["jie", r, o] => Inst::Jie(parse_reg(r), o.parse().unwrap()),
        ["jio", r, o] => Inst::Jio(parse_reg(r), o.parse().unwrap()),
        _             => unreachable!()
    }
}

fn run_inst(i : &Inst, regs : &mut [usize ; 2], pc : &mut isize)
{
    match i
    {
        Inst::Hlf(r)    => { *pc += 1; regs[*r] /= 2 },
        Inst::Tpl(r)    => { *pc += 1; regs[*r] *= 3 },
        Inst::Inc(r)    => { *pc += 1; regs[*r] += 1 },
        Inst::Jmp(   o) =>   *pc += o,
        Inst::Jie(r, o) =>   *pc += if regs[*r] % 2 == 0 { *o } else { 1 },
        Inst::Jio(r, o) =>   *pc += if regs[*r]     == 1 { *o } else { 1 }
    }
}
