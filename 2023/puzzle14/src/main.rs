use std::collections::HashMap;
use aoc::direction::Direction;

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
        let mut rocks = platform.rocks.iter()
                                      .filter(|&(_, &v)| v)
                                      .map(|(&p, _)| p)
                                      .collect::<Vec<Pos>>();
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
    rocks:  HashMap<Pos, bool>,
    width:  i32,
    height: i32
}

impl Platform
{
    fn tilt(&mut self, dir : Direction)
    {

        for mut rock in self.rocks.extract_if(|_, &mut v| v)
                                  .map(|(p, _)| p)
                                  .collect::<Vec<Pos>>()
                                  .into_iter()
        {
            let obstructed = |(x, y) : Pos| !(0 .. self.width).contains(&x)  ||
                                            !(0 .. self.height).contains(&y) ||
                                             self.rocks.contains_key(&(x, y));

            while !obstructed(rock)
            {
                rock = dir.step(rock)
            }

            while obstructed(rock)
            {
                rock = dir.opposite().step(rock)
            }

            self.rocks.insert(rock, true);
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
        self.rocks.iter()
                  .filter(|&(_, &v)| v)
                  .map(|((_, y), _)| self.height - y)
                  .sum()
    }

    fn parse(s : &str) -> Platform
    {
        let mut rocks  = HashMap::new();
        let mut width  = 0;
        let mut height = 0;

        for (l, y) in s.lines().zip(0..)
        {
            for (b, x) in l.bytes().zip(0..)
            {
                match b
                {
                    b'#' => { rocks.insert((x, y), false); },
                    b'O' => { rocks.insert((x, y), true);  },
                    _    => ()
                }

                width  = width.max(x+1);
                height = height.max(y+1);
            }
        }

        Platform { rocks, width, height }
    }
}
