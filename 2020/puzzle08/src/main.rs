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
            Inst::Nop(k) => input[j] = Inst::Jmp(k),
            Inst::Jmp(k) => input[j] = Inst::Nop(k),
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
    Nop(i64),
    Acc(i64),
    Jmp(i64)
}

impl Inst
{
    fn parse(s : &str) -> Inst
    {
        let mut it = s.split_whitespace();
        let op     = it.next().unwrap();
        let arg    = it.next().unwrap().parse().unwrap();

        match op
        {
            "nop" => Inst::Nop(arg),
            "acc" => Inst::Acc(arg),
            "jmp" => Inst::Jmp(arg),
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
                Some(Inst::Nop(_)) => { pc += 1 },
                Some(Inst::Acc(k)) => { pc += 1; acc += k },
                Some(Inst::Jmp(k)) => { pc += k }
            }
        }
    }
}
