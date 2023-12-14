use std::{ ops::Range, collections::{ HashSet, HashMap }};

fn main()
{
    let mut platform = Platform::parse(include_str!("../input.txt"));

    platform.tilt(Direction::North);
    println!("{}", platform.load());

    const CYCLES : usize = 1_000_000_000;
    let mut visited      = HashMap::new();
    let mut loads        = HashMap::new();

    for n in 0 .. CYCLES
    {
        let mut rocks = platform.rocks.iter().copied().collect::<Vec<Pos>>();
        rocks.sort_unstable();

        if let Some(m) = visited.insert(rocks, n)
        {
            println!("{}", loads[&(m + (CYCLES - n) % (n - m))]);
            break
        }

        loads.insert(n, platform.load());
        platform.spin();
    }
}

type Pos = (i32, i32);

struct Platform
{
    rows:  Range<i32>,
    cols:  Range<i32>,
    walls: HashSet<Pos>,
    rocks: HashSet<Pos>
}

impl Platform
{
    fn tilt(&mut self, dir : Direction)
    {
        for mut rock in self.rocks.drain().collect::<Vec<Pos>>().into_iter()
        {
            let obstructed = |p : Pos| !self.rows.contains(&p.1) ||
                                       !self.cols.contains(&p.0) ||
                                        self.walls.contains(&p)  ||
                                        self.rocks.contains(&p);

            while !obstructed(rock)
            {
                rock = dir.offset(rock)
            }

            while obstructed(rock)
            {
                rock = dir.opposite().offset(rock)
            }

            self.rocks.insert(rock);
        }
    }

    fn spin(&mut self)
    {
        self.tilt(Direction::North);
        self.tilt(Direction::West);
        self.tilt(Direction::South);
        self.tilt(Direction::East);
    }

    fn load(&self) -> i32
    {
        self.rocks.iter().map(|(_, y)| self.cols.end - y).sum()
    }

    fn parse(s : &str) -> Platform
    {
        let mut walls  = HashSet::new();
        let mut rocks  = HashSet::new();
        let mut width  = 0;
        let mut height = 0;

        for (l, y) in s.lines().zip(0..)
        {
            for (b, x) in l.bytes().zip(0..)
            {
                match b
                {
                    b'#' => { walls.insert((x, y)); },
                    b'O' => { rocks.insert((x, y)); },
                    _    => ()
                }

                width  = width.max(x+1);
                height = height.max(y+1);
            }
        }

        Platform { rows: 0 .. width, cols: 0 .. height, walls, rocks }
    }
}

#[derive(Copy, Clone)]
enum Direction { North, West, South, East }

impl Direction
{
    fn offset(self, (x, y) : Pos) -> Pos
    {
        match self
        {
            Direction::North => (x, y-1),
            Direction::West  => (x-1, y),
            Direction::South => (x, y+1),
            Direction::East  => (x+1, y)
        }
    }

    fn opposite(self) -> Direction
    {
        match self
        {
            Direction::North => Direction::South,
            Direction::West  => Direction::East,
            Direction::South => Direction::North,
            Direction::East  => Direction::West
        }
    }
}
