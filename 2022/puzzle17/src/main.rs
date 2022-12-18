use std::{ ops::{ Add, AddAssign }, collections::{ HashSet, HashMap }};

fn main()
{
    let mut cycle_map = HashMap::new();
    let mut rocks = (1 ..).zip(rockfall());
    println!("{}", rocks.by_ref().take(2022)
                        .fold(0, |_, (ix, (height, key))|
                        {
                            cycle_map.insert(key, (ix, height));
                            height
                        }));

    for (curr_ix, (curr_height, key)) in rocks.by_ref()
    {
        if let Some((prev_ix, prev_height)) = cycle_map.insert(key, (curr_ix, curr_height))
        {
            let cycle_len  = curr_ix           - prev_ix;
            let rocks_left = 1_000_000_000_000 - curr_ix;
            println!("{}", curr_height + (curr_height - prev_height) * (rocks_left / cycle_len)
                         + rocks.take((rocks_left % cycle_len) as usize)
                                .last().map(|(_, (h, _))| h - curr_height)
                                .unwrap_or(0));

            break
        }
    }
}

type CycleKey = (Shape, [i64 ; 7]);

fn rockfall() -> impl Iterator<Item = (i64, CycleKey)>
{
    let mut jet_cycle = include_str!("../input.txt").trim_end().bytes().cycle().filter_map(|b| match b
    {
        b'<' => Some(Direction::Left),
        b'>' => Some(Direction::Right),
        _    => None
    });

    let mut height  = 0;
    let mut settled = HashSet::new();

    Shape::cycle().map(move |shape|
    {
        let mut rock_pos = Pos { x: 2, y: height+4 };

        'fall: loop
        {
            for dir in jet_cycle.by_ref().take(1).chain(std::iter::once(Direction::Down))
            {
                rock_pos += dir.offset();

                let collision = shape.points().any(|mut pos|
                {
                    pos += rock_pos;
                    !(0 .. 7).contains(&pos.x) || pos.y < 1 || settled.contains(&pos)
                });

                if collision
                {
                    rock_pos += dir.opposite().offset();
                    if let Direction::Down = dir { break 'fall }
                }
            }
        }

        height = shape.points().fold(height, |height, mut pos|
        {
            pos += rock_pos;
            settled.insert(pos);
            height.max(pos.y)
        });

        let mut depths = [0 ; 7];
        for(d, x) in depths.iter_mut().zip(0 ..)
        {
            for pos in (0 ..= height).rev().map(move |y| Pos { x, y })
            {
                if settled.contains(&pos) { *d = height - pos.y; break }
            }
        }

        (height, (shape, depths))
    })
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Pos
{
    x: i64,
    y: i64
}

impl Add for Pos
{
    type Output = Pos;

    fn add(self, other : Pos) -> Pos
    {
        Pos
        {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl AddAssign for Pos
{
    fn add_assign(&mut self, other : Pos)
    {
        *self = self.add(other);
    }
}

#[derive(Clone, Copy)]
enum Direction { Left, Right, Up, Down }

impl Direction
{
    fn opposite(self) -> Direction
    {
        match self
        {
            Direction::Left  => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up    => Direction::Down,
            Direction::Down  => Direction::Up
        }
    }

    fn offset(self) -> Pos
    {
        match self
        {
            Direction::Left  => Pos { x: -1, y:  0 },
            Direction::Right => Pos { x:  1, y:  0 },
            Direction::Up    => Pos { x:  0, y:  1 },
            Direction::Down  => Pos { x:  0, y: -1 }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Shape { Flat, Cross, Corner, Tall, Square }

impl Shape
{
    fn points(self) -> impl Iterator<Item = Pos>
    {
        match self
        {
            Shape::Flat   => [Pos { x: 0, y: 0 },
                              Pos { x: 1, y: 0 },
                              Pos { x: 2, y: 0 },
                              Pos { x: 3, y: 0 }].as_slice(),
            Shape::Cross  => [Pos { x: 1, y: 0 },
                              Pos { x: 0, y: 1 },
                              Pos { x: 1, y: 1 },
                              Pos { x: 2, y: 1 },
                              Pos { x: 1, y: 2 }].as_slice(),
            Shape::Corner => [Pos { x: 0, y: 0 },
                              Pos { x: 1, y: 0 },
                              Pos { x: 2, y: 0 },
                              Pos { x: 2, y: 1 },
                              Pos { x: 2, y: 2 }].as_slice(),
            Shape::Tall   => [Pos { x: 0, y: 0 },
                              Pos { x: 0, y: 1 },
                              Pos { x: 0, y: 2 },
                              Pos { x: 0, y: 3 }].as_slice(),
            Shape::Square => [Pos { x: 0, y: 0 },
                              Pos { x: 1, y: 0 },
                              Pos { x: 0, y: 1 },
                              Pos { x: 1, y: 1 }].as_slice()
        }
        .iter()
        .copied()
    }

    fn cycle() -> impl Iterator<Item = Shape>
    {
        [Shape::Flat,
         Shape::Cross,
         Shape::Corner,
         Shape::Tall,
         Shape::Square].iter()
                       .copied()
                       .cycle()
    }
}
