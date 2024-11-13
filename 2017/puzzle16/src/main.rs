use std::collections::VecDeque;

fn main()
{
    let moves = include_str!("../input.txt").trim_end().split(',').map(Move::parse).collect::<Vec<Move>>();

    let mut progs = (b'a' ..).take(16).collect::<VecDeque<u8>>();
    moves.iter().for_each(|m| m.step(&mut progs));
    println!("{}", std::str::from_utf8(progs.make_contiguous()).unwrap());

    for i in 1 ..
    {
        if progs.iter().is_sorted()
        {
            for _ in 0 .. (1_000_000_000 - i) % i
            {
                moves.iter().for_each(|m| m.step(&mut progs));
            }
            println!("{}", std::str::from_utf8(progs.make_contiguous()).unwrap());
            break
        }
        moves.iter().for_each(|m| m.step(&mut progs));
    }
}

#[derive(Copy, Clone)]
enum Move
{
    Spin(usize),
    Exchange(usize, usize),
    Partner(u8, u8)
}

impl Move
{
    fn parse(s : &str) -> Move
    {
        match s.split_at(1)
        {
            ("s",           rest) => Move::Spin(rest.parse().unwrap()),
            (m@("x" | "p"), rest) =>
            {
                let (a, b) = rest.split_once('/').unwrap();
                if m == "x"
                {
                    Move::Exchange(a.parse().unwrap(), b.parse().unwrap())
                }
                else
                {
                    Move::Partner(a.as_bytes()[0], b.as_bytes()[0])
                }
            },
            _ => unreachable!()
        }
    }

    fn step(self, progs : &mut VecDeque<u8>)
    {
        match self
        {
            Move::Spin(k)        => progs.rotate_right(k),
            Move::Exchange(a, b) => progs.swap(a, b),
            Move::Partner(a, b)  =>
            {
                let a = progs.iter().position(|&p| p == a).unwrap();
                let b = progs.iter().position(|&p| p == b).unwrap();
                progs.swap(a, b)
            }
        }
    }
}
