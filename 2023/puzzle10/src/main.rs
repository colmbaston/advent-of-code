use std::collections::{ HashMap, HashSet };

fn main()
{
    let (start, pipes) = parse_pipes(include_str!("../input.txt"));

    let mut prev    = start;
    let mut next    = pipes[&start].connections(start)[0];
    let mut visited = std::iter::once(start).collect::<HashSet<Pos>>();
    loop
    {
        let current = next;
        if current == start { break }

        visited.insert(current);
        next = pipes[&next].connections(next).into_iter()
                           .find(|&p| p != prev)
                           .unwrap();
        prev = current;
    }
    println!("{}", visited.len() / 2);

    let mut count = 0;
    let (min_x, min_y, max_x, max_y) = aoc::bounds::bounds_2d(visited.iter()).unwrap();
    for y in min_y+1 ..= max_y-1
    {
        let mut parity = false;
        for x in min_x ..= max_x
        {
            if visited.contains(&(x, y))
            {
                if matches!(pipes[&(x, y)], Segment::NS
                                          | Segment::NE
                                          | Segment::NW) { parity = !parity }
            }
            else if parity { count += 1 }
        }
    }
    println!("{count}");
}

type Pos = (i32, i32);

#[derive(Copy, Clone)]
enum Segment { NS, EW, NE, NW, SW, SE }

fn parse_pipes(s : &str) -> (Pos, HashMap<Pos, Segment>)
{
    let mut start = None;
    let mut pipes = HashMap::new();

    for (l, y) in s.lines().zip(0..)
    {
        for (b, x) in l.bytes().zip(0..)
        {
            if let Ok(segment) = b.try_into() { pipes.insert((x, y), segment); }
            else if b == b'S'                 { start = Some((x, y))           }
        }
    }

    let start       = start.unwrap();
    let start_conns = Segment::NS.connections(start).into_iter()
                                 .chain(Segment::EW.connections(start))
                                 .map(|p| pipes.get(&p).copied()
                                               .filter(|s : &Segment| s.connections(p)
                                                                       .into_iter()
                                                                       .any(|q| q == start))
                                               .is_some())
                                 .collect::<Vec<bool>>();

    pipes.insert(start, match start_conns[..]
    {
        [true,  true,  false, false] => Segment::NS,
        [false, false, true,  true ] => Segment::EW,
        [true,  false, true,  false] => Segment::NE,
        [true,  false, false, true ] => Segment::NW,
        [false, true,  false, true ] => Segment::SW,
        [false, true,  true,  false] => Segment::SE,
        _                            => unreachable!()
    });

    (start, pipes)
}

impl Segment
{
    fn connections(self, (x, y) : Pos) -> [Pos ; 2]
    {
        match self
        {
            Segment::NS => [(x, y-1), (x, y+1)],
            Segment::EW => [(x-1, y), (x+1, y)],
            Segment::NE => [(x, y-1), (x+1, y)],
            Segment::NW => [(x, y-1), (x-1, y)],
            Segment::SE => [(x, y+1), (x+1, y)],
            Segment::SW => [(x, y+1), (x-1, y)]
        }
    }
}

impl TryFrom<u8> for Segment
{
    type Error = ();

    fn try_from(b : u8) -> Result<Segment, ()>
    {
        match b
        {
            b'|' => Ok(Segment::NS),
            b'-' => Ok(Segment::EW),
            b'L' => Ok(Segment::NE),
            b'J' => Ok(Segment::NW),
            b'7' => Ok(Segment::SW),
            b'F' => Ok(Segment::SE),
            _    => Err(())
        }
    }
}
