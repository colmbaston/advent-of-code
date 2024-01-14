use std::collections::HashMap;

fn main()
{
    let input = include_str!("../input.txt").trim_end().parse::<i32>().unwrap();

    let (x, y) = spiral_pos(input);
    println!("{}", x.abs() + y.abs());

    let mut spiral = HashMap::new();
    spiral.insert((0, 0), 1);

    for i in 2 ..
    {
        let (x, y) = spiral_pos(i);
        let value  = (x-1 ..= x+1).flat_map(|ax| (y-1 ..= y+1).map(move |ay| (ax, ay)))
                                  .filter(|&(ax, ay)| x != ax || y != ay)
                                  .map(|pos| spiral.get(&pos).copied().unwrap_or(0))
                                  .sum();

        if value > input { println!("{value}"); break }
        spiral.insert((x, y), value);
    }
}

fn spiral_pos(i : i32) -> (i32, i32)
{
    let sqrt   = (1 ..).step_by(2).take_while(|&k| i > k*k).last().unwrap();
    let square = sqrt * sqrt;
    let layer  = sqrt / 2 + 1;
    let pos    = i - square - 1;
    let size   = (sqrt+2).pow(2) - square;

    let mut x = layer;
    let mut y = 1 - layer + pos % (size / 4);
    for _ in 0 .. 4 * pos / size { std::mem::swap(&mut x, &mut y); x = -x }
    (x, y)
}
