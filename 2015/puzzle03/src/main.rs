use std::collections::HashSet;

fn main()
{
    let input = include_str!("../input.txt");

    let mut sx     = 0;
    let mut sy     = 0;
    let mut houses = HashSet::new();

    for b in input.bytes()
    {
        houses.insert((sx, sy));

        match b
        {
            b'^' => sy += 1,
            b'v' => sy -= 1,
            b'>' => sx += 1,
            b'<' => sx -= 1,
            _    => break
        }
    }
    println!("{}", houses.len());

    sx         = 0;
    sy         = 0;
    let mut rx = 0;
    let mut ry = 0;
    houses.clear();

    for (i, b) in input.bytes().enumerate()
    {
        let (x, y) = if i % 2 != 0 { (&mut rx, &mut ry) } else { (&mut sx, &mut sy) };
        houses.insert((*x, *y));

        match b
        {
            b'^' => *y += 1,
            b'v' => *y -= 1,
            b'>' => *x += 1,
            b'<' => *x -= 1,
            _    => break
        }
    }
    println!("{}", houses.len());
}
