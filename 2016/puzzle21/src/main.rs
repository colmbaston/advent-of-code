fn main()
{
    let ops = include_str!("../input.txt").lines().map(Op::parse).collect::<Vec<Op>>();

    let mut pass = "abcdefgh".as_bytes().to_vec();
    ops.iter().for_each(|op| op.apply(&mut pass));
    println!("{}", std::str::from_utf8(&pass).unwrap());

    pass = "fbgdceah".as_bytes().to_vec();
    ops.iter().rev().for_each(|op| op.inverse(&mut pass));
    println!("{}", std::str::from_utf8(&pass).unwrap());
}

enum Op
{
    Swap(usize, usize),
    SwapLetter(u8, u8),
    Rotate(bool, usize),
    RotateLetter(u8),
    Reverse(usize, usize),
    Move(usize, usize)
}

impl Op
{
    fn apply(&self, pass : &mut Vec<u8>)
    {
        let len = pass.len();

        match *self
        {
            Op::Swap(i, j)       => pass.swap(i, j),
            Op::SwapLetter(a, b) => pass.iter_mut().for_each(|c| if *c == a { *c = b } else if *c == b { *c = a }),
            Op::Rotate(right, i) => if right { pass.rotate_right(i) } else { pass.rotate_left(i) }
            Op::RotateLetter(a)  => { let i = pass.iter().position(|&b| b == a).unwrap(); pass.rotate_right((i + 1 + (i >= 4) as usize) % len) },
            Op::Reverse(i, j)    => pass[i ..= j].reverse(),
            Op::Move(i, j)       => { let a = pass.remove(i); pass.insert(j, a) }
        }
    }

    fn inverse(&self, pass : &mut Vec<u8>)
    {
        let len = pass.len();

        match *self
        {
            Op::Rotate(right, i) => Op::Rotate(!right, i).apply(pass),
            Op::RotateLetter(a)  =>
            {
                let i = pass.iter().position(|&b| b == a).unwrap();
                pass.rotate_left((0 .. len).find(|j|
                {
                    let k = (len + i - j) % len;
                    i == (k + k + 1 + (k >= 4) as usize) % len
                })
                .unwrap());
            },
            Op::Move(i, j) => Op::Move(j, i).apply(pass),
            _              => self.apply(pass)
        }
    }

    fn parse(s : &str) -> Op
    {
        match s.split_whitespace().collect::<Vec<&str>>()[..]
        {
            ["swap", "position", i, "with", "position", j]           => Op::Swap(i.parse().unwrap(), j.parse().unwrap()),
            ["swap", "letter",   a, "with", "letter",   b]           => Op::SwapLetter(a.bytes().next().unwrap(), b.bytes().next().unwrap()),
            ["rotate", dir@("right" | "left"), i, "step" | "steps"]  => Op::Rotate(dir == "right", i.parse().unwrap()),
            ["rotate", "based", "on", "position", "of", "letter", a] => Op::RotateLetter(a.bytes().next().unwrap()),
            ["reverse", "positions", i, "through",        j]         => Op::Reverse(i.parse().unwrap(), j.parse().unwrap()),
            ["move",    "position",  i, "to", "position", j]         => Op::Move(i.parse().unwrap(), j.parse().unwrap()),
            _                                                        => unreachable!()
        }
    }
}
