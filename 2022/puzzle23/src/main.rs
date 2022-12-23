use std::{ ops::Range, collections::{ HashSet, HashMap }};

fn main()
{
    let mut proposals = HashMap::new();
    let mut elves     = include_str!("../input.txt").lines().zip(0 ..)
                                                    .flat_map(|(l, y)| l.bytes().zip(0 ..)
                                                                        .filter_map(move |(b, x)| (b == b'#').then_some(Pos { x, y })))
                                                    .collect::<HashSet<Pos>>();

    for round in 1 ..
    {
        for &elf in elves.iter()
        {
            let adjacent = elf.adjacent();
            if adjacent[.. 8].iter().any(|pos| elves.contains(pos))
            {
                for dir in Direction::enumerate().cycle().skip((round - 1) % 4).take(4)
                {
                    let scan = &adjacent[dir.adjacent_range()];
                    if !scan.iter().any(|pos| elves.contains(pos))
                    {
                        proposals.entry(scan[1]).or_insert((elf, 0)).1 += 1;
                        break
                    }
                }
            }
        }

        if proposals.is_empty()
        {
            println!("{round}");
            break
        }

        for (dest, (elf, count)) in proposals.drain()
        {
            if count == 1
            {
                elves.remove(&elf);
                elves.insert(dest);
            }
        }

        if round == 10
        {
            let mut min_corner = elves.iter().next().cloned().unwrap_or(Pos { x: 0, y: 0 });
            let mut max_corner = min_corner;

            for &elf in elves.iter()
            {
                min_corner.x = min_corner.x.min(elf.x);
                min_corner.y = min_corner.y.min(elf.y);
                max_corner.x = max_corner.x.max(elf.x);
                max_corner.y = max_corner.y.max(elf.y);
            }

            println!("{}", (1 + max_corner.x - min_corner.x) *
                           (1 + max_corner.y - min_corner.y) - elves.len() as i16);
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Pos
{
    x: i16,
    y: i16
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
    fn enumerate() -> impl Iterator<Item = Direction> + Clone
    {
        [Direction::North,
         Direction::South,
         Direction::West,
         Direction::East].into_iter()
    }

    fn adjacent_range(self) -> Range<usize>
    {
        match self
        {
            Direction::North => 0 .. 3,
            Direction::East  => 2 .. 5,
            Direction::South => 4 .. 7,
            Direction::West  => 6 .. 9
        }
    }
}
