use std::{ cmp::Reverse, collections::{ HashSet, BinaryHeap }};
use puzzle24::{ Pos, Blizzard, lcm };

fn main()
{
    let (width, height, mut blizzards) = Blizzard::parse(include_str!("../input.txt"));

    let mut cycle = Vec::new();
    for _ in 0 .. lcm(width, height)
    {
        cycle.push(blizzards.iter_mut().map(|blizz|
        {
            let pos = blizz.pos;
            blizz.step(width, height);
            pos
        })
        .collect::<HashSet<Pos>>());
    }
    drop(blizzards);

    let top    = Pos { x: 0,       y: -1     };
    let bottom = Pos { x: width-1, y: height };
    let valid  = |pos : Pos, minute : usize| pos == top || pos == bottom
                                         || ((0 .. width).contains(&pos.x)  &&
                                             (0 .. height).contains(&pos.y) &&
                                             !cycle[minute % cycle.len()].contains(&pos));

    let mut start_time = 0;
    let mut queue      = BinaryHeap::new();
    let mut visited    = HashSet::new();
    for (trip, (start, finish)) in [(top, bottom), (bottom, top)].into_iter().cycle().enumerate()
    {
        queue.clear();
        visited.clear();
        queue.push((Reverse((start_time + (width + height) as usize, start_time)), start));

        while let Some((Reverse((_, mut minute)), pos)) = queue.pop()
        {
            if !visited.insert((pos, minute % cycle.len())) { continue }
            if pos == finish { start_time = minute; break }

            minute += 1;
            queue.extend(pos.moves().filter(|&pos| valid(pos, minute)).map(|pos|
            {
                (Reverse((minute + pos.manhattan(finish) as usize, minute)), pos)
            }))
        }

        if trip == 0
        {
            println!("{start_time}");
        }
        else if trip == 2
        {
            println!("{start_time}");
            break
        }
    }
}
