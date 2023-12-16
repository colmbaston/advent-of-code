use std::collections::HashMap;

fn main()
{
    let cont = Contraption::parse(include_str!("../input.txt"));

    let mut energised = HashMap::new();
    cont.energise((0, 0), Dir::E, &mut energised);
    println!("{}", energised.len());

    let vertical   = (0 .. cont.width ).flat_map(|x| [((x,             0), Dir::S),
                                                      ((x, cont.height-1), Dir::N)]);
    let horizontal = (0 .. cont.height).flat_map(|y| [((0,             y), Dir::E),
                                                      ((cont.width-1,  y), Dir::W)]);

    println!("{}", vertical.chain(horizontal).map(|(pos, dir)|
    {
        energised.clear();
        cont.energise(pos, dir, &mut energised);
        energised.len()
    })
    .max().unwrap());
}

type Pos = (i32, i32);

struct Contraption
{
    grid:   HashMap<Pos, Tile>,
    width:  i32,
    height: i32
}

enum Tile { MirrorF, MirrorB, SplitterV, SplitterH }

impl Contraption
{
    fn parse(s : &str) -> Contraption
    {
        let mut grid   = HashMap::new();
        let mut width  = 0;
        let mut height = 0;

        for (l, y) in s.lines().zip(0..)
        {
            for (b, x) in l.bytes().zip(0..)
            {
                match b
                {
                    b'/'  => { grid.insert((x, y), Tile::MirrorF);   },
                    b'\\' => { grid.insert((x, y), Tile::MirrorB);   },
                    b'|'  => { grid.insert((x, y), Tile::SplitterV); },
                    b'-'  => { grid.insert((x, y), Tile::SplitterH); },
                    b'.'  => (),
                    _     => unreachable!()
                }

                width  = width.max(x+1);
                height = height.max(y+1);
            }
        }

        Contraption { grid, width, height }
    }

    fn energise(&self, mut pos : Pos, mut dir : Dir, visited : &mut HashMap<Pos, u8>)
    {
        while (0 .. self.width).contains(&pos.0) && (0 .. self.height).contains(&pos.1)
        {
            let set = visited.entry(pos).or_insert(0);
            if *set & dir.bit() != 0 { break }
            *set |= dir.bit();

            match (self.grid.get(&pos), dir)
            {
                (Some(Tile::MirrorF), _) => { dir = dir.reflect()            },
                (Some(Tile::MirrorB), _) => { dir = dir.reflect().opposite() },

                (Some(Tile::SplitterV), Dir::E | Dir::W) |
                (Some(Tile::SplitterH), Dir::N | Dir::S) =>
                {
                    dir = dir.reflect();
                    self.energise(pos, dir, visited);
                    dir = dir.opposite();
                },

                _ => *set |= dir.opposite().bit()
            }

            pos = dir.offset(pos);
        }
    }
}

#[derive(Clone, Copy)]
enum Dir { N, S, E, W }

impl Dir
{
    fn offset(self, (x, y) : Pos) -> Pos
    {
        match self
        {
            Dir::N => (x, y-1),
            Dir::S => (x, y+1),
            Dir::E => (x+1, y),
            Dir::W => (x-1, y)
        }
    }

    fn reflect(self) -> Dir
    {
        match self
        {
            Dir::N => Dir::E,
            Dir::S => Dir::W,
            Dir::E => Dir::N,
            Dir::W => Dir::S
        }
    }

    fn opposite(self) -> Dir
    {
        match self
        {
            Dir::N => Dir::S,
            Dir::S => Dir::N,
            Dir::E => Dir::W,
            Dir::W => Dir::E
        }
    }

    fn bit(self) -> u8
    {
        1 << self as u8
    }
}
