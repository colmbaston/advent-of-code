use std::collections::HashMap;
use aoc::direction::Direction;

fn main()
{
    let (grid,  path) = include_str!("../input.txt").split_once("\n\n").unwrap();
    let (robot, grid) = parse_grid(grid);
    let path          = parse_path(path);
    println!("{}", solve(robot, grid.clone(), path.iter().copied()));

    let mut grid_two = HashMap::new();
    for ((x, y), b) in grid.into_iter()
    {
        match b
        {
            b'#' => grid_two.extend([((x*2, y), b'#'), ((x*2+1, y), b'#')]),
            b'O' => grid_two.extend([((x*2, y), b'['), ((x*2+1, y), b']')]),
            _    => unreachable!()
        }
    }
    println!("{}", solve((robot.0*2, robot.1), grid_two, path.into_iter()));
}

type Pos = (i32, i32);

fn parse_grid(s : &str) -> (Pos, HashMap<Pos, u8>)
{
    let mut robot = (0, 0);
    let mut grid  = HashMap::new();
    for (l, y) in s.lines().zip(0 ..)
    {
        for (b, x) in l.bytes().zip(0 ..)
        {
            match b
            {
                b'@'        => robot = (x, y),
                b'#' | b'O' => { grid.insert((x, y), b); },
                _           => ()
            }
        }
    }

    (robot, grid)
}

fn parse_path(s : &str) -> Vec<Direction>
{
    s.lines().flat_map(|l| l.bytes()).map(|b| match b
    {
        b'^' => Direction::North,
        b'>' => Direction::East,
        b'v' => Direction::South,
        b'<' => Direction::West,
        _    => unreachable!()
    })
    .collect()
}

fn solve(mut robot : Pos, mut grid : HashMap<Pos, u8>, path : impl Iterator<Item = Direction>) -> i32
{
    let mut boxes = HashMap::new();
    for dir in path
    {
        boxes.clear();
        if box_chain(robot, dir, &grid, &mut boxes)
        {
            for (pos, _) in boxes.iter()
            {
                grid.remove(pos);
            }
            for (pos, b) in boxes.drain()
            {
                grid.insert(dir.step(pos), b);
            }
            robot = dir.step(robot);
        }
    }

    grid.into_iter()
        .filter(|(_, b)| *b == b'O' || *b == b'[')
        .map(|((x, y), _)| x + 100 * y)
        .sum::<i32>()
}

fn box_chain(pos : Pos, dir : Direction, grid : &HashMap<Pos, u8>, boxes : &mut HashMap<Pos, u8>) -> bool
{
    let next = dir.step(pos);
    match grid.get(&next)
    {
        None       => true,
        Some(b'#') => false,
        Some(&b)   =>
        {
            if b == b'O' || matches!(dir, Direction::East | Direction::West)
            {
                boxes.insert(next, b);
                box_chain(next, dir, grid, boxes)
            }
            else
            {
                let (left, right) = if b == b'[' { (next, Direction::East.step(next)) }
                                    else         { (Direction::West.step(next), next) };

                (boxes.insert(left,  b'[').is_some() || box_chain(left,  dir, grid, boxes)) &&
                (boxes.insert(right, b']').is_some() || box_chain(right, dir, grid, boxes))
            }
        }
    }
}
