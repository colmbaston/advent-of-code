fn main()
{
    let (a, b, c, prog) = parse_prog(include_str!("../input.txt"));
    println!("{}", run(&prog, [a, b, c]).into_iter().map(|o| o.to_string()).collect::<Vec<String>>().join(","));
    println!("{}", solve(&prog, prog.len()-1, 0).unwrap());
}

fn solve(prog : &[u8], i : usize, a : u64) -> Option<u64>
{
    (0 .. 8).find_map(|k|
    {
        let a = a << 3 | k;
        (run(prog, [a, 0, 0]) == prog[i ..]).then(|| (i == 0).then_some(a)
                                                             .or_else(|| solve(prog, i-1, a)))
                                            .flatten()
    })
}

fn parse_prog(s : &str) -> (u64, u64, u64, Vec<u8>)
{
    let     s  = s.strip_prefix("Register A: ").unwrap();
    let (a, s) = s.split_once("\nRegister B: ").unwrap();
    let (b, s) = s.split_once("\nRegister C: ").unwrap();
    let (c, s) = s.split_once("\n\nProgram: ").unwrap();

    (a.parse().unwrap(),
     b.parse().unwrap(),
     c.parse().unwrap(),
     s.trim_end().split(',').map(|t| t.parse().unwrap()).collect())
}

fn run(prog : &[u8], mut regs : [u64 ; 3]) -> Vec<u8>
{
    let mut ip     = 0;
    let mut output = Vec::new();

    while ip+1 < prog.len()
    {
        let inst = prog[ip];
        let oper = prog[ip+1];
        ip += 2;

        match inst
        {
            0 => regs[0] >>= combo_operand(oper, &regs),
            1 => regs[1] ^= oper as u64,
            2 => regs[1] = combo_operand(oper, &regs) & 7,
            3 => if regs[0] != 0 { ip = oper as usize },
            4 => regs[1] ^= regs[2],
            5 => output.push(combo_operand(oper, &regs) as u8 & 7),
            6 => regs[1] = regs[0] >> combo_operand(oper, &regs),
            7 => regs[2] = regs[0] >> combo_operand(oper, &regs),
            _ => unreachable!()
        }
    }

    output
}

fn combo_operand(oper : u8, regs : &[u64 ; 3]) -> u64
{
    match oper
    {
        0 ..= 3 => oper as u64,
        4 ..= 6 => regs[oper as usize - 4],
        _       => unreachable!()
    }
}
