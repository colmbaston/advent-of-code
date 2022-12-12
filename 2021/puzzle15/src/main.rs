fn main()
{
    let cave = include_str!("../input.txt").lines().map(|l| l.bytes().map(|b| b - b'0').collect::<Vec<u8>>()).collect::<Vec<Vec<u8>>>();
    let ly   = cave.len();
    let lx   = cave.first().map(|row| row.len()).unwrap_or(0);

    type Pos = (usize, usize);

    let risk       = |&(x, y) : &Pos| cave.get(y).and_then(|row : &Vec<u8>| row.get(x));
    let orthogonal = |&(x, y) : &Pos|
    {
        [(x+1, y), (x, y+1)].into_iter()
                            .chain(x.checked_sub(1).map(|x| (x, y)).into_iter())
                            .chain(y.checked_sub(1).map(|y| (x, y)).into_iter())
    };

    let target   = |&(x, y) : &Pos| x == lx-1 && y == ly-1;
    let adjacent = |pos     : &Pos| orthogonal(pos).filter_map(|next| risk(&next).map(|&r| (next, r as usize)));
    if let Some(risk) = aoc::pathfinding::dijkstra(std::iter::once((0, 0)), target, adjacent) { println!("{risk}") }

    let target   = |&(x, y) : &Pos| x == 5*lx-1 && y == 5*ly-1;
    let adjacent = |pos     : &Pos|
    {
        orthogonal(pos).filter_map(|pos@(x, y)| if x < 5*lx && y < 5*ly
          { risk(&(x % lx, y % ly)).map(|&r| (pos, (r as usize + (x / lx + y / ly) - 1) % 9 + 1)) }
        else
          { None })
    };
    if let Some(risk) = aoc::pathfinding::dijkstra(std::iter::once((0, 0)), target, adjacent) { println!("{risk}") }
}
