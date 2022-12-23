use std::{ ops::RangeInclusive, collections::{ HashSet, HashMap }};

fn main()
{
    let mut proposals = HashMap::new();
    let mut elves     = include_str!("../input.txt").lines().zip(0 ..)
                                                    .flat_map(|(l, y)| l.bytes().zip(0 ..)
                                                                        .filter_map(move |(b, x)| (b == b'#').then_some(Pos { x, y })))
                                                    .collect::<HashSet<Pos>>();

    for round in 0 ..
    {
        for &elf in elves.iter()
        {
            let adjacent = elf.adjacent();
            if adjacent.iter().any(|pos| elves.contains(pos))
            {
                for dir in Direction::cycle().skip(round).take(4)
                {
                    let scan = &adjacent[dir.adjacent_range()];
                    if !scan.iter().any(|pos| elves.contains(pos))
                    {
                        proposals.entry(scan[1]).or_insert_with(Vec::new).push(elf);
                        break
                    }
                }
            }
        }

        if proposals.is_empty()
        {
            println!("{}", round+1);
            break
        }

        for (dest, proposers) in proposals.drain()
        {
            if let [elf] = proposers[..]
            {
                elves.remove(&elf);
                elves.insert(dest);
            }
        }

        if round == 9
        {
            let mut min_corner = Pos { x: 0, y: 0 };
            let mut max_corner = min_corner;

            for &elf in elves.iter()
            {
                min_corner.x = min_corner.x.min(elf.x);
                min_corner.y = min_corner.y.min(elf.y);
                max_corner.x = max_corner.x.max(elf.x);
                max_corner.y = max_corner.y.max(elf.y);
            }

            println!("{}", (1 + max_corner.x - min_corner.x) *
                           (1 + max_corner.y - min_corner.y) - elves.len() as i32);
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Pos
{
    x: i32,
    y: i32
}

#[derive(Clone, Copy)]
enum Direction { North, South, West, East }

impl Pos
{
    fn adjacent(self) -> [Pos ; 9]
    {
        let Pos { x, y } = self;

        [Pos { x: x-1, y: y-1 },
         Pos { x     , y: y-1 },
         Pos { x: x+1, y: y-1 },
         Pos { x: x+1, y      },
         Pos { x: x+1, y: y+1 },
         Pos { x,      y: y+1 },
         Pos { x: x-1, y: y+1 },
         Pos { x: x-1, y      },
         Pos { x: x-1, y: y-1 }]
    }
}

impl Direction
{
    fn cycle() -> impl Iterator<Item = Direction>
    {
        [Direction::North,
         Direction::South,
         Direction::West,
         Direction::East].into_iter()
                         .cycle()
    }

    fn adjacent_range(self) -> RangeInclusive<usize>
    {
        match self
        {
            Direction::North => 0 ..= 2,
            Direction::East  => 2 ..= 4,
            Direction::South => 4 ..= 6,
            Direction::West  => 6 ..= 8
        }
    }
}
