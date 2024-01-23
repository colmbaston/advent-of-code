use std::collections::HashMap;
use aoc::direction::Direction;

fn main()
{
    let (mut pos, mut grid) = parse_grid(include_str!("../input.txt"));
    {
        let mut dir   = Direction::North;
        let mut pos   = pos;
        let mut grid  = grid.clone();
        let mut count = 0;
        for _ in 0 .. 10_000
        {
            if matches!(burst_one(&mut dir, &mut pos, &mut grid), State::Infected) { count += 1 }
        }
        println!("{count}");
    }

    let mut dir   = Direction::North;
    let mut count = 0;
    for _ in 0 .. 10_000_000
    {
        if matches!(burst_two(&mut dir, &mut pos, &mut grid), State::Infected) { count += 1 }
    }
    println!("{count}");
}

type Pos = (i32, i32);

#[derive(Copy, Clone)]
enum State { Clean, Weakened, Infected, Flagged }

fn parse_grid(s : &str) -> (Pos, HashMap<Pos, State>)
{
    let mut max_x = 0;
    let mut max_y = 0;
    let mut grid  = HashMap::new();
    for (l, y) in s.lines().zip(0 ..)
    {
        for (b, x) in l.bytes().zip(0 ..)
        {
            if b == b'#' { grid.insert((x, y), State::Infected); }
            max_x = max_x.max(x)
        }
        max_y = max_y.max(y)
    }
    ((max_x / 2, max_y / 2), grid)
}

fn burst_one(dir : &mut Direction, pos : &mut Pos, grid : &mut HashMap<Pos, State>) -> State
{
    let current = grid.entry(*pos).or_insert(State::Clean);
    match current
    {
        State::Clean => { *dir = dir.anticlockwise(); *current = State::Infected },
        _            => { *dir = dir.clockwise();     *current = State::Clean    }
    }
    *pos = dir.step(*pos);
    *current
}

fn burst_two(dir : &mut Direction, pos : &mut Pos, grid : &mut HashMap<Pos, State>) -> State
{
    let current = grid.entry(*pos).or_insert(State::Clean);
    match current
    {
        State::Clean    => { *dir = dir.anticlockwise(); *current = State::Weakened },
        State::Weakened => {                             *current = State::Infected },
        State::Infected => { *dir = dir.clockwise();     *current = State::Flagged  },
        State::Flagged  => { *dir = dir.opposite();      *current = State::Clean    },
    }
    *pos = dir.step(*pos);
    *current
}
