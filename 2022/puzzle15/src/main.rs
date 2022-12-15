use std::{ ops::RangeInclusive, collections::HashSet };

mod sensor;
use sensor::Sensor;

fn main()
{
    let (sensors, beacons) = include_str!("../input.txt").lines().fold((Vec::new(), HashSet::new()), |(mut sensors, mut beacons), l|
    {
        if let Some((sensor, beacon)) = Sensor::parse(l)
        {
            sensors.push(sensor);
            beacons.insert(beacon);
        }
        (sensors, beacons)
    });

    let mut one      = None;
    let mut two      = None;
    let mut coverage = Vec::new();
    let mut merged   = Vec::new();
    let distress_x   = 0 ..= 4_000_000;
    let distress_y   = 0 ..= 4_000_000;
    for y in distress_y
    {
        coverage.extend(sensors.iter().filter_map(|s| { let c = s.coverage_y(y); (!c.is_empty()).then_some(c) }));
        coverage.sort_unstable_by_key(|r| *r.start());
        merged.clear();
        merge_ranges(coverage.drain(..), &mut merged);

        if y == 2_000_000
        {
            let covered = merged.iter().map(range_size).sum::<usize>();
            let beacons = beacons.iter().filter(|b| b.y == y).count();
            one         = Some(covered - beacons);
        }

        for gap in merged.windows(2).map(|w| *w[0].end()+1 ..= *w[1].start()-1)
        {
            let x = gap.end();
            if distress_x.contains(x)
            {
                two = Some(*x as usize * 4_000_000 + y as usize);
            }
        }

        if let Some((one, two)) = one.and_then(|one| two.map(|two| (one, two)))
        {
            println!("{one}");
            println!("{two}");
            break
        }
    }
}

fn merge_ranges(ranges : impl Iterator<Item = RangeInclusive<i32>>, merged : &mut Vec<RangeInclusive<i32>>)
{
    for range in ranges
    {
        match merged.last_mut()
        {
            None       => merged.push(range),
            Some(last) => if *range.start() <= *last.end()+1
            {
                *last = *last.start() ..= *last.end().max(range.end())
            }
            else
            {
                merged.push(range);
            }
        }
    }
}

fn range_size(range : &RangeInclusive<i32>) -> usize
{
    if range.is_empty() { 0 } else { 1 + (range.end() - range.start()) as usize }
}
