#![feature(array_windows)]

use std::{ ops::RangeInclusive, collections::HashSet };

mod sensor;
use sensor::Sensor;

fn main()
{
    let (sensors, beacons) = include_str!("../input.txt").lines().fold((Vec::new(), HashSet::new()), |(mut ss, mut bs), l|
    {
        if let Some((s, b)) = Sensor::parse(l) { ss.push(s); bs.insert(b); }
        (ss, bs)
    });

    let mut coverage = Vec::new();
    let mut merged   = Vec::new();

    merge_coverage(2_000_000, sensors.iter(), &mut coverage, &mut merged);
    let covered = merged.iter().map(|r| if r.is_empty() { 0 } else { 1 + (*r.end() - *r.start()) as usize }).sum::<usize>();
    let beacons = beacons.into_iter().filter(|b| b.y == 2_000_000).count();
    println!("{}", covered - beacons);

    'outer: for y in 0 ..= 4_000_000
    {
        merge_coverage(y, sensors.iter(), &mut coverage, &mut merged);
        for gap in merged.array_windows().map(|[a, b]| a.end()+1 ..= b.start()-1)
        {
            let x = gap.start();
            if x == gap.end() && (0 ..= 4_000_000).contains(x)
            {
                println!("{}", *x as u64 * 4_000_000 + y as u64);
                break 'outer
            }
        }
    }
}

fn merge_coverage<'a>(row : i32, sensors : impl Iterator<Item = &'a Sensor>, coverage : &mut Vec<RangeInclusive<i32>>, merged : &mut Vec<RangeInclusive<i32>>)
{
    coverage.extend(sensors.filter_map(|s| { let r = s.row_coverage(row); (!r.is_empty()).then_some(r) }));
    coverage.sort_unstable_by_key(|r| *r.start());

    merged.clear();
    for range in coverage.drain(..)
    {
        match merged.last_mut()
        {
            Some(last) if *range.start() <= *last.end()+1 =>
            {
                *last = *last.start() ..= *last.end().max(range.end())
            },
            _ => merged.push(range)
        }
    }
}
