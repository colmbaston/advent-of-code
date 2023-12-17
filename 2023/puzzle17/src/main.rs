use aoc::direction::Direction;

fn main()
{
    let grid   = parse_grid(include_str!("../input.txt"));
    let target = (grid[0].len()-1, grid.len()-1);

    println!("{}", aoc::pathfinding::dijkstra(State::inits(),
                                              |s| s.pos == target,
                                              |s| s.adjacent(0, 3, &grid)).unwrap());
    println!("{}", aoc::pathfinding::dijkstra(State::inits(),
                                              |s| s.pos == target && 4 <= s.straight,
                                              |s| s.adjacent(4, 10, &grid)).unwrap());
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
    dir:      Direction,
    straight: u32
}

impl State
{
    fn inits() -> impl Iterator<Item = State>
    {
        [Direction::East,
         Direction::South].into_iter()
                          .map(|dir| State { pos: (0, 0), dir, straight: 0 })
    }

    fn adjacent(self, min : u32, max : u32, grid : &[Vec<u8>]) -> impl Iterator<Item = (State, u32)> + '_
    {
        let mut next = Vec::new();
        if self.straight < max  { next.push((self.dir,   self.straight+1)) }
        if min <= self.straight { next.push((self.dir.clockwise(),     1));
                                  next.push((self.dir.anticlockwise(), 1)) }

        next.into_iter()
            .filter_map(move |(dir, straight)| dir.checked_step(self.pos)
                                                  .and_then(|pos| grid.get(pos.1)
                                                                      .and_then(|row| row.get(pos.0))
                                                                      .map(|&loss| (pos, loss as u32)))
                                                  .map(|(pos, loss)| (State { pos, dir, straight }, loss)))
    }
}
