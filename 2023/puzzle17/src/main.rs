fn main()
{
    let grid   = parse_grid(include_str!("../input.txt"));
    let target = (grid[0].len()-1, grid.len()-1);

    println!("{}", aoc::pathfinding::dijkstra(State::inits(),
                                              |s| s.pos == target,
                                              |s| s.adjacent(0, 3)
                                                   .filter_map(|s| grid.get(s.pos.1)
                                                                       .and_then(|row| row.get(s.pos.0))
                                                                       .map(|&loss| (s, loss as u32)))).unwrap());

    println!("{}", aoc::pathfinding::dijkstra(State::inits(),
                                              |s| s.pos == target && 4 <= s.straight,
                                              |s| s.adjacent(4, 10)
                                                   .filter_map(|s| grid.get(s.pos.1)
                                                                       .and_then(|row| row.get(s.pos.0))
                                                                       .map(|&loss| (s, loss as u32)))).unwrap());
}

type Pos = (usize, usize);

fn parse_grid(s : &str) -> Vec<Vec<u8>>
{
    s.lines()
     .map(|l| l.bytes().map(|b| b - b'0').collect())
     .collect()
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct State
{
    pos:      Pos,
    dir:      Dir,
    straight: u32
}

impl State
{
    fn inits() -> impl Iterator<Item = State>
    {
        [Dir::E, Dir::S].into_iter()
                        .map(|dir| State { pos: (0, 0), dir, straight: 0 })
    }

    fn adjacent(self, min : u32, max : u32) -> impl Iterator<Item = State>
    {
        let mut next = Vec::new();
        if self.straight < max  { next.push((self.dir,   self.straight+1)) }
        if min <= self.straight { next.push((self.dir.clockwise(),     1));
                                  next.push((self.dir.anticlockwise(), 1)) }

        next.into_iter()
            .filter_map(move |(dir, straight)| dir.offset(self.pos)
                                                  .map(|pos| State { pos, dir, straight }))
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Dir { N, E, S, W }

impl Dir
{
    fn from_u8(b : u8) -> Dir
    {
        match b % 4
        {
            0 => Dir::N,
            1 => Dir::E,
            2 => Dir::S,
            3 => Dir::W,
            _ => unreachable!()
        }
    }

    fn clockwise(self) -> Dir
    {
        Dir::from_u8((self as u8).wrapping_add(1))
    }

    fn anticlockwise(self) -> Dir
    {
        self.clockwise().opposite()
    }

    fn opposite(self) -> Dir
    {
        Dir::from_u8((self as u8).wrapping_add(2))
    }

    fn offset(self, (x, y) : Pos) -> Option<Pos>
    {
        match self
        {
            Dir::N => y.checked_sub(1).map(|y| (x, y)),
            Dir::E => x.checked_add(1).map(|x| (x, y)),
            Dir::S => y.checked_add(1).map(|y| (x, y)),
            Dir::W => x.checked_sub(1).map(|x| (x, y))
        }
    }
}
