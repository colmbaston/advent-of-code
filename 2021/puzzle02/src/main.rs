fn main()
{
    let input = include_str!("../input.txt").lines().map(parse_instruction).collect::<Vec<(Direction, i32)>>();

    let mut position = 0;
    let mut depth    = 0;
    for (d, n) in input.iter()
    {
        match d
        {
            Direction::Forward => position += n,
            Direction::Up      => depth    -= n,
            Direction::Down    => depth    += n
        }
    }
    println!("{}", position * depth);

    let mut aim = 0;
    position    = 0;
    depth       = 0;
    for (d, n) in input.iter()
    {
        match d
        {
            Direction::Forward => { position += n; depth += aim * n },
            Direction::Up      =>   aim      -= n,
            Direction::Down    =>   aim      += n
        }
    }
    println!("{}", position * depth);
}

enum Direction { Forward, Up, Down }

fn parse_instruction(s : &str) -> (Direction, i32)
{
    let direction = match s.bytes().next()
    {
        Some(b'f') => Direction::Forward,
        Some(b'u') => Direction::Up,
        Some(b'd') => Direction::Down,
        _          => unreachable!()
    };

    (direction, s[s.find(' ').unwrap()+1 ..].parse().unwrap())
}
