#![feature(slice_as_chunks, iter_next_chunk)]
use std::ops::{ Range, Neg };

fn main()
{
    let mut input   = include_str!("../input.txt").split("\n\n");
    let seeds       = input.next().unwrap()
                           .strip_prefix("seeds: ").unwrap()
                           .split_whitespace()
                           .map(|k| k.parse::<i64>().unwrap())
                           .collect::<Vec<i64>>();
    let mut almenac = Almenac(input.map(Map::parse)
                                   .collect::<Vec<Map>>());

    println!("{}", seeds.iter()
                        .map(|&s| almenac.apply(s))
                        .min().unwrap());

    let seeds = seeds.as_chunks().0.iter()
                     .map(|&[s1, s2]| s1 .. s1+s2)
                     .collect::<Vec<Range<i64>>>();

    almenac.invert();
    let mut endpoints = Vec::new();
    for m in almenac.0.iter()
    {
        endpoints.iter_mut().for_each(|e| *e = m.apply(*e));
        endpoints.extend(m.endpoints());
    }

    almenac.invert();
    println!("{}", endpoints.into_iter()
                            .filter(|e| seeds.iter().any(|s| s.contains(e)))
                            .chain(seeds.iter().map(|s| s.start))
                            .map(|s| almenac.apply(s))
                            .min().unwrap())
}

struct Almenac(Vec<Map>);

impl Almenac
{
    fn apply(&self, n : i64) -> i64
    {
        self.0.iter().fold(n, |a, m| m.apply(a))
    }

    fn invert(&mut self)
    {
        self.0.reverse();
        self.0.iter_mut().for_each(Map::invert)
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
        self.0.iter_mut().for_each(Mapping::invert)
    }

    fn endpoints(&self) -> impl Iterator<Item = i64> + '_
    {
        std::iter::once(0).chain(self.0.iter()
                                       .flat_map(|m| [m.range.start + m.offset,
                                                      m.range.end   + m.offset]))
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
        (!self.range.contains(&n)).then_some(n).ok_or(n + self.offset)
    }

    fn invert(&mut self)
    {
        self.range  = self.range.start + self.offset .. self.range.end + self.offset;
        self.offset = self.offset.neg()
    }

    fn parse(s : &str) -> Mapping
    {
        let [dest, source, len] = s.split_whitespace()
                                   .map(|k| k.parse::<i64>().unwrap())
                                   .next_chunk().unwrap();

        Mapping { range:  source .. source+len, offset: dest - source }
    }
}
