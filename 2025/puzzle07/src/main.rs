use std::collections::HashMap;

fn main()
{
    let grid = include_str!("../input.txt").lines()
                                           .map(|l| l.as_bytes())
                                           .collect::<Vec<&[u8]>>();

    let mut splits = 0;
    let mut beams  = HashMap::new();
    let mut next   = HashMap::new();
    beams.insert(grid[0].iter().position(|&b| b == b'S').unwrap(), 1);
    for row in grid.iter()
    {
        for (beam, count) in beams.drain()
        {
            if let b'^' = row[beam]
            {
                splits += 1;
                *next.entry(beam-1).or_insert(0) += count;
                *next.entry(beam+1).or_insert(0) += count;
            }
            else
            {
                *next.entry(beam).or_insert(0) += count;
            }
        }
        std::mem::swap(&mut beams, &mut next);
    }
    println!("{splits}");
    println!("{}", beams.into_values().sum::<u64>());
}
