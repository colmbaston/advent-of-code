#![feature(iter_next_chunk)]
use std::collections::HashMap;

fn main()
{
    let (workflows, parts) = include_str!("../input.txt").split_once("\n\n").unwrap();
    let workflows = workflows.lines().map(Workflow::parse).collect::<HashMap<&str, Workflow>>();
    let parts     = parts.lines().map(Part::parse).collect::<Vec<Part>>();

    println!("{}", parts.iter().filter(|p| p.accepted(&workflows)).map(|p| p.rating()).sum::<u32>());
}

struct Part([u32 ; 4]);

impl Part
{
    fn parse(s : &str) -> Part
    {
        Part(s.strip_suffix('}').unwrap()
              .split(',')
              .map(|s| s.split_once('=').unwrap().1
                        .parse().unwrap())
              .next_chunk().unwrap())
    }

    fn rating(&self) -> u32
    {
        self.0.iter().sum()
    }

    fn accepted<'a>(&self, workflows : &HashMap<&'a str, Workflow<'a>>) -> bool
    {
        let mut state = "in";
        let next      = || workflows.get(state)
                                    .and_then(|w| w.run(self)
                                                   .map(|s| { state = s; s }));

        std::iter::from_fn(next).last() == Some("A")
    }
}

struct Workflow<'a>(Vec<Test<'a>>);

impl<'a> Workflow<'a>
{
    fn parse(s : &'a str) -> (&'a str, Workflow<'a>)
    {
        let (state, s) = s.split_once('{').unwrap();
        (state, Workflow(s.strip_suffix('}').unwrap()
                          .split(',')
                          .map(Test::parse)
                          .collect()))
    }

    fn run(&self, part : &Part) -> Option<&'a str>
    {
        self.0.iter().find_map(|test| test.run(part))
    }
}

enum Test<'a>
{
    BinOp(Op, usize, u32, &'a str),
    Unconditional(&'a str)
}

impl<'a> Test<'a>
{
    fn parse(s : &'a str) -> Test<'a>
    {
        let (init, s) = s.split_at(s.find(|c : char| !c.is_ascii_alphabetic()).unwrap_or(s.len()));

        let ix = match (init, s)
        {
            (_,  "") => return Test::Unconditional(init),
            ("x", _) => 0,
            ("m", _) => 1,
            ("a", _) => 2,
            ("s", _) => 3,
            _        => unreachable!()
        };

        let op = match s.as_bytes()[0]
        {
            b'<' => Op::LT,
            b'>' => Op::GT,
            _    => unreachable!()
        };

        let (k, dest) = s[1..].split_once(':').unwrap();
        Test::BinOp(op, ix, k.parse().unwrap(), dest)
    }

    fn run(&self, part : &Part) -> Option<&'a str>
    {
        match self
        {
            Test::BinOp(op, ix, k, dest) => op.apply(part.0[*ix], *k).then_some(dest),
            Test::Unconditional(dest)    => Some(dest)
        }
    }
}

#[derive(Copy, Clone)]
enum Op { LT, GT }

impl Op
{
    fn apply<T : Ord>(self, a : T, b : T) -> bool
    {
        match self
        {
            Op::LT => a < b,
            Op::GT => a > b
        }
    }
}
