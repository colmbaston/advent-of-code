use std::collections::HashSet;

fn main()
{
    let mut input = include_str!("../input.txt").lines().map(Inst::parse).collect::<Vec<_>>();

    println!("{:?}", Inst::run(&input).unwrap_or_else(|acc| acc));

    for j in 0 .. input.len()
    {
        let i = input.get(j).unwrap().clone();

        match i
        {
            Inst::NOP(k) => input[j] = Inst::JMP(k),
            Inst::JMP(k) => input[j] = Inst::NOP(k),
            _            => continue
        }

        if let Ok(acc) = Inst::run(&input)
        {
            println!("{}", acc);
            break
        }

        input[j] = i;
    }
}

#[derive(Clone)]
enum Inst
{
    NOP(i64),
    ACC(i64),
    JMP(i64)
}

impl Inst
{
    fn parse(s : &str) -> Inst
    {
        let mut i = s.split_whitespace();
        let op    = i.next().unwrap();
        let arg   = i.next().unwrap().parse().unwrap();

        match op
        {
            "nop" => Inst::NOP(arg),
            "acc" => Inst::ACC(arg),
            "jmp" => Inst::JMP(arg),
            _     => unreachable!()
        }
    }

    fn run(prog : &[Inst]) -> Result<i64, i64>
    {
        let mut pc      = 0;
        let mut acc     = 0;
        let mut visited = HashSet::new();

        loop
        {
            if !visited.insert(pc) { break Err(acc) }
            match prog.get(pc as usize)
            {
                None               => break Ok(acc),
                Some(Inst::NOP(_)) => { pc += 1 },
                Some(Inst::ACC(k)) => { pc += 1; acc += k },
                Some(Inst::JMP(k)) => { pc += k }
            }
        }
    }
}
