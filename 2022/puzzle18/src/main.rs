use std::collections::HashSet;

mod pos;
use pos::Pos;

fn main()
{
    let mut min = Pos { x: i32::MAX, y: i32::MAX, z: i32::MAX };
    let mut max = Pos { x: i32::MIN, y: i32::MIN, z: i32::MIN };
    let shape   = include_str!("../input.txt").lines().filter_map(Pos::parse).inspect(|&pos|
    {
        min = min.min_corner(pos);
        max = max.max_corner(pos)
    })
    .collect::<HashSet<Pos>>();

    min = min + Pos { x: -1, y: -1, z: -1 };
    max = max + Pos { x:  1, y:  1, z:  1 };

    let mut queue    = vec![min];
    let mut exterior = HashSet::new();
    while let Some(pos) = queue.pop()
    {
        if pos.in_rect(min, max) && !shape.contains(&pos) && exterior.insert(pos)
        {
            queue.extend(pos.adjacents())
        }
    }
    drop(queue);

    let (one, two) = shape.iter().flat_map(|pos| pos.adjacents()).fold((0, 0), |(one, two), pos|
    {
        (one + !shape.contains(&pos)    as u32,
         two +  exterior.contains(&pos) as u32)
    });
    println!("{one}");
    println!("{two}");
}
