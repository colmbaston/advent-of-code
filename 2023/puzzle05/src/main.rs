#![feature(slice_as_chunks, iter_next_chunk)]
use std::ops::Range;

fn main()
{
    let mut input   = include_str!("../input.txt").split("\n\n");
    let seeds       = input.next().unwrap()
                           .strip_prefix("seeds: ").unwrap()
                           .split_whitespace()
                           .map(|k| k.parse::<i64>().unwrap())
                           .collect::<Vec<i64>>();
    let mut almenac = Almenac(input.map(Map::parse).collect::<Vec<Map>>());

    println!("{}", seeds.iter()
                        .map(|&seed| almenac.apply(seed))
                        .min().unwrap());

    almenac.invert();
    let seeds = seeds.as_chunks().0.iter()
                     .map(|&[s1, s2]| s1 .. s1+s2)
                     .collect::<Vec<Range<i64>>>();

    println!("{}", (0 ..).find(|&loc|
    {

        let seed = almenac.apply(loc);
        seeds.iter().any(|r| r.contains(&seed))
    })
    .unwrap());
}

struct Almenac(Vec<Map>);

impl Almenac
{
    fn apply(&self, n : i64) -> i64
    {
        self.0.iter().fold(n, |a, map| map.apply(a))
    }

    fn invert(&mut self)
    {
        self.0.reverse();
        self.0.iter_mut().for_each(Map::invert);
    }
}

struct Map(Vec<Mapping>);

impl Map
{
    fn apply(&self, n : i64) -> i64
    {
        self.0.iter().try_fold(n, |a, m| m.apply(a))
                     .unwrap_or_else(|err| err)
    }

    fn invert(&mut self)
    {
        self.0.reverse();
        self.0.iter_mut().for_each(Mapping::invert)
    }

    fn parse(s : &str) -> Map
    {
        Map(s.lines().skip(1).map(Mapping::parse).collect())
    }
}

struct Mapping
{
    range:  Range<i64>,
    offset: i64
}

impl Mapping
{
    fn apply(&self, n : i64) -> Result<i64, i64>
    {
        (!self.range.contains(&n)).then_some(n)
                                  .ok_or(n + self.offset)
    }

    fn invert(&mut self)
    {
        self.range  = self.range.start + self.offset .. self.range.end + self.offset;
        self.offset = -self.offset;
    }

    fn parse(s : &str) -> Mapping
    {
        let [dest, source, len] = s.split_whitespace()
                                   .map(|k| k.parse::<i64>().unwrap())
                                   .next_chunk().unwrap();

        Mapping
        {
            range:  source .. source+len,
            offset: dest - source
        }
    }
}
