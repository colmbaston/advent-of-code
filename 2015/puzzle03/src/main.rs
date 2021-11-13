use std::collections::HashSet;

fn main()
{
    let input = include_str!("../input.txt");

    let mut sx     = 0;
    let mut sy     = 0;
    let mut houses = HashSet::new();
    houses.insert((0, 0));

    for b in input.bytes()
    {
        match b
        {
            b'^' => sy += 1,
            b'v' => sy -= 1,
            b'>' => sx += 1,
            b'<' => sx -= 1,
            _    => break
        }

        houses.insert((sx, sy));
    }
    println!("{}", houses.len());

    sx         = 0;
    sy         = 0;
    let mut rx = 0;
    let mut ry = 0;
    houses.clear();
    houses.insert((0, 0));

    for (i, b) in input.bytes().enumerate()
    {
        let (x, y) = if i % 2 == 0 { (&mut sx, &mut sy) } else { (&mut rx, &mut ry) };

        match b
        {
            b'^' => *y += 1,
            b'v' => *y -= 1,
            b'>' => *x += 1,
            b'<' => *x -= 1,
            _    => break
        }

        houses.insert((*x, *y));
    }
    println!("{}", houses.len());
}
