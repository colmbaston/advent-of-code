use std::collections::HashMap;

fn main()
{
    let input = include_str!("../input.txt").lines().map(Instruction::parse).collect::<Vec<_>>();

    let mut mem = HashMap::new();
    let mut or  = 0x000000000;
    let mut and = 0xfffffffff;
    for i in input.iter()
    {
        match i
        {
            Instruction::SetMask(a, o)     => { and = *a; or = *o },
            Instruction::SetMem(addr, val) => { mem.insert(*addr, val & and | or); }
        }
    }
    println!("{}", mem.values().sum::<u64>());

    mem.clear();
    let mut floating = 0x000000000;
    or               = 0x000000000;
    for i in input.iter()
    {
        match i
        {
            Instruction::SetMask(a, o)   => { or = *o; floating = *a ^ or },
            Instruction::SetMem(addr, x) => { set_mem_floating(&mut mem, (addr | or) & !floating, floating, 0, *x) }
        }
    }
    println!("{}", mem.values().sum::<u64>());
}

fn set_mem_floating(mem : &mut HashMap<u64, u64>, addr : u64, floating : u64, start : u64, val : u64)
{
    mem.insert(addr, val);
    for bit in (start .. 36).filter(|bit| floating & 1 << bit != 0)
    {
        set_mem_floating(mem, addr | 1 << bit, floating, bit+1, val)
    }
}

enum Instruction
{
    SetMask(u64, u64),
    SetMem(u64, u64)
}

impl Instruction
{
    fn parse(s : &str) -> Instruction
    {
        match s.strip_prefix("mask = ")
        {
            None =>
            {
                let s = &s[4..];
                let (a, s) = s.split_at(s.find(|c : char| !c.is_ascii_digit()).unwrap_or(s.len()));

                Instruction::SetMem(a.parse().unwrap(), s[4..].parse().unwrap())
            },
            Some(s) =>
            {
                let (and, or) = s.bytes().fold((0x0, 0x0), |(and, or), b|
                {
                    match b
                    {
                        b'0' => (and << 1,     or << 1),
                        b'1' => (and << 1 | 1, or << 1 | 1),
                        b'X' => (and << 1 | 1, or << 1),
                        _    => unreachable!()
                    }
                });

                Instruction::SetMask(and, or)
            }
        }
    }
}
