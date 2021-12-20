use std::collections::HashSet;

fn main()
{
    let (rules, mut canvas) = parse_canvas(include_str!("../input.txt"));

    for i in [2, 48].into_iter()
    {
        for j in 0 .. i
        {
            canvas = enhance(&rules, &canvas, j % 2 != 0 && rules[0]);
        }
        println!("{}", canvas.len());
    }
}

fn parse_canvas(s : &str) -> (Vec<bool>, HashSet<(i32, i32)>)
{
    let mut i = s.split("\n\n");

    let     rules  = i.next().unwrap().bytes().map(|b| b == b'#').collect();
    let mut canvas = HashSet::new();

    for (l, y) in i.next().unwrap().lines().zip(0 ..)
    {
        for (b, x) in l.bytes().zip(0 ..)
        {
            if b == b'#' { canvas.insert((x, y)); }
        }
    }

    (rules, canvas)
}

fn enhance(rules : &[bool], canvas : &HashSet<(i32, i32)>, parity : bool) -> HashSet<(i32, i32)>
{
    let mut next = HashSet::new();
    if let Some((min_x, min_y, max_x, max_y)) = aoc::bounds::bounds_2d(canvas.iter())
    {
        for y in min_y-1 ..= max_y+1
        {
            for x in min_x-1 ..= max_x+1
            {
                let mut index = (y-1 ..= y+1).flat_map(|y| (x-1 ..= x+1).map(move |x| (x, y)))
                                             .fold(0, |a, p| a << 1 | canvas.contains(&p) as usize);

                if parity
                {
                    index |= match min_x - x { 0 => 0b100_100_100, 1 => 0b110_110_110, _ => 0 };
                    index |= match x - max_x { 0 => 0b001_001_001, 1 => 0b011_011_011, _ => 0 };
                    index |= match min_y - y { 0 => 0b111_000_000, 1 => 0b111_111_000, _ => 0 };
                    index |= match y - max_y { 0 => 0b000_000_111, 1 => 0b000_111_111, _ => 0 };
                }

                if rules[index] { next.insert((x, y)); }
            }
        }
    }
    next
}
