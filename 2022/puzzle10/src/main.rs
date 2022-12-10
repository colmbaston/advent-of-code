fn main()
{
    let mut signal  = 0;
    let mut display = String::new();
    let prog        = include_str!("../input.txt").lines().filter_map(Inst::parse);
    for ((reg, cycle), pixel) in exec(prog).zip(1 ..).zip((0 .. 40).cycle())
    {
        display.push(if reg.abs_diff(pixel) <= 1 { '#' } else { ' ' });
        if      pixel == 19 { signal += reg * cycle }
        else if pixel == 39 { display.push('\n') }
    }
    println!("{signal}");
    print!("{display}");
}

#[derive(Clone, Copy)]
enum Inst { NoOp, AddX(i32) }

impl Inst
{
    fn parse(s : &str) -> Option<Inst>
    {
        let mut words = s.split_whitespace();

        match words.next()?
        {
            "noop" => Some(Inst::NoOp),
            "addx" => Some(Inst::AddX(words.next()?.parse().ok()?)),
            _      => None
        }
    }

    fn cycles(&self) -> usize
    {
        match self
        {
            Inst::NoOp    => 1,
            Inst::AddX(_) => 2
        }
    }
}

fn exec(prog : impl Iterator<Item = Inst>) -> impl Iterator<Item = i32>
{
    prog.scan(1, |reg, inst|
    {
        let reg_during = *reg;
        if let Inst::AddX(k) = inst { *reg += k }
        Some(std::iter::repeat(reg_during).take(inst.cycles()))
    })
    .flatten()
}
