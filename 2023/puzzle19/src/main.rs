#![feature(iter_next_chunk)]
use std::{ ops::RangeInclusive, collections::HashMap };

fn main()
{
    let (workflows, parts) = include_str!("../input.txt").split_once("\n\n").unwrap();
    let workflows = workflows.lines().map(Workflow::parse).collect::<HashMap<&str, Workflow>>();

    println!("{}", parts.lines().map(Part::parse)
                        .filter(|p| p.split(&workflows).len() == 1)
                        .map(|p| p.0.iter().map(|r| r.start()).sum::<u32>())
                        .sum::<u32>());

    const RATINGS : RangeInclusive<u32> = 1 ..= 4000;
    println!("{}", Part([RATINGS ; 4]).split(&workflows).iter()
                                      .map(Part::combinations)
                                      .sum::<u64>());
}

#[derive(Clone)]
struct Part([RangeInclusive<u32> ; 4]);

impl Part
{
    fn parse(s : &str) -> Part
    {
        let singleton = |k| k ..= k;
        Part(s.strip_suffix('}').unwrap()
              .split(',')
              .map(|s| singleton(s.split_once('=').unwrap().1
                                  .parse().unwrap()))
              .next_chunk().unwrap())
    }

    fn split<'a>(&self, workflows : &HashMap<&'a str, Workflow<'a>>) -> Vec<Part>
    {
        let mut states   = vec![(self.clone(), "in")];
        let mut buffer   = Vec::new();
        let mut accepted = Vec::new();

        while !states.is_empty()
        {
            for (part, state) in states.drain(..)
            {
                match state
                {
                    "A" => accepted.push(part),
                    "R" => continue,
                    _   => buffer.extend(workflows[state].split(&part))
                }
            }
            std::mem::swap(&mut states, &mut buffer)
        }

        accepted
    }

    fn combinations(&self) -> u64
    {
        self.0.iter()
              .map(|r| 1 + (r.end() - r.start()) as u64)
              .product()
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

    fn split(&self, part : &Part) -> impl Iterator<Item = (Part, &'a str)> + '_
    {
        self.0.iter().scan(Some(part.clone()), |next, test|
        {
            match next
            {
                None       => None,
                Some(part) =>
                {
                    let (t, f) = test.split(part);
                    *next = f;
                    Some(t.map(|t| (t, test.dest())).into_iter())
                }
            }
        })
        .flatten()
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

    fn split(&self, part : &Part) -> (Option<Part>, Option<Part>)
    {
        match self
        {
            Test::BinOp(op, ix, k, _) =>
            {
                let (t, f) = op.split(&part.0[*ix], *k);
                ((!t.is_empty()).then(|| { let mut part = part.clone(); part.0[*ix] = t; part }),
                 (!f.is_empty()).then(|| { let mut part = part.clone(); part.0[*ix] = f; part }))
            },
            Test::Unconditional(_) => (Some(part.clone()), None)
        }
    }

    fn dest(&self) -> &'a str
    {
        match self
        {
            Test::BinOp(_, _, _, dest) => dest,
            Test::Unconditional(dest)  => dest
        }
    }
}

#[derive(Copy, Clone)]
enum Op { LT, GT }

impl Op
{
    fn split(self, r : &RangeInclusive<u32>, k : u32) -> (RangeInclusive<u32>, RangeInclusive<u32>)
    {
        match self
        {
            Op::LT => (*r.start().min(&k)     ..= *r.end().min(&(k-1)),
                       *r.start().max(&k)     ..= *r.end().max(&(k-1))),
            Op::GT => (*r.start().max(&(k+1)) ..= *r.end().max(&k),
                       *r.start().min(&(k+1)) ..= *r.end().min(&k))
        }
    }
}
