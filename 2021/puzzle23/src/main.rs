use std::cmp::Reverse;
use std::collections::{ BinaryHeap, HashSet };

fn main()
{
    let mut input = Burrow::parse(include_str!("../input.txt"));
    println!("{}", input.dijkstra().unwrap());

    input.depth += 2;
    input.rooms[Amphipod::A.index()].insert(1, Amphipod::D);
    input.rooms[Amphipod::A.index()].insert(1, Amphipod::D);
    input.rooms[Amphipod::B.index()].insert(1, Amphipod::C);
    input.rooms[Amphipod::B.index()].insert(1, Amphipod::B);
    input.rooms[Amphipod::C.index()].insert(1, Amphipod::B);
    input.rooms[Amphipod::C.index()].insert(1, Amphipod::A);
    input.rooms[Amphipod::D.index()].insert(1, Amphipod::A);
    input.rooms[Amphipod::D.index()].insert(1, Amphipod::C);
    println!("{}", input.dijkstra().unwrap());
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Amphipod { A, B, C, D }

impl Amphipod
{
    fn from_byte(b : u8) -> Option<Amphipod>
    {
        match b
        {
            b'A' => Some(Amphipod::A),
            b'B' => Some(Amphipod::B),
            b'C' => Some(Amphipod::C),
            b'D' => Some(Amphipod::D),
            _    => None
        }
    }

    fn index(self) -> usize
    {
        match self
        {
            Amphipod::A => 0,
            Amphipod::B => 1,
            Amphipod::C => 2,
            Amphipod::D => 3
        }
    }

    fn from_index(i : usize) -> Amphipod
    {
        match i
        {
            0 => Amphipod::A,
            1 => Amphipod::B,
            2 => Amphipod::C,
            3 => Amphipod::D,
            _ => unreachable!()
        }
    }

    fn energy(&self) -> usize
    {
        match self
        {
            Amphipod::A =>    1,
            Amphipod::B =>   10,
            Amphipod::C =>  100,
            Amphipod::D => 1000,
        }
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Burrow
{
    hallway: [Option<Amphipod> ; 11],
    rooms:   [Vec<Amphipod>    ;  4],
    depth:   usize
}

impl Burrow
{
    fn parse(s : &str) -> Burrow
    {
        let hallway   : [Option<Amphipod> ; 11] = [None ; 11];
        let mut rooms : [Vec<Amphipod>    ;  4] = Default::default();
        let mut amphipods                       = s.bytes().filter_map(Amphipod::from_byte);

        'outer: loop
        {
            for room in rooms.iter_mut().rev()
            {
                match amphipods.next_back()
                {
                    None    => break 'outer,
                    Some(a) => room.push(a)
                }
            }
        }

        let depth = rooms[0].len();
        Burrow { hallway, rooms, depth }
    }

    fn dijkstra(&self) -> Option<usize>
    {
        let mut queue   = BinaryHeap::new();
        let mut visited = HashSet::new();

        queue.push((Reverse(0), self.clone()));
        while let Some((Reverse(cost), burrow)) = queue.pop()
        {
            if visited.contains(&burrow) { continue          }
            if burrow.is_ideal()         { return Some(cost) }

            queue.extend(burrow.ascending_moves()
                               .chain(burrow.descending_moves())
                               .map(|(b, c)| (Reverse(cost+c), b)));

            visited.insert(burrow);
        }

        None
    }

    fn is_ideal(&self) -> bool
    {
        self.hallway.iter().all(Option::is_none) &&
        self.rooms.iter().enumerate().all(|(ri, room)|
        {
            let a = Amphipod::from_index(ri);
            room.iter().all(|&b| a == b)
        })
    }

    fn ascending_moves(&self) -> impl Iterator<Item = (Burrow, usize)> + '_
    {
        self.rooms.iter().enumerate().filter_map(|(ri, room)| (!room.is_empty()).then(|| ri)).flat_map(move |ri|
        {
            let hi = 2 * (ri + 1);
            [0, 1, 3, 5, 7, 9, 10].into_iter().filter_map(move |dest|
            {
                let path = if hi < dest { &self.hallway[hi+1 ..= dest] } else { &self.hallway[dest .. hi] };
                path.iter().all(Option::is_none).then(||
                {
                    let mut burrow       = self.clone();
                    let room             = &mut burrow.rooms[ri];
                    let amphipod         = room.pop().unwrap();
                    let cost             = amphipod.energy() * (path.len() + burrow.depth - room.len());
                    burrow.hallway[dest] = Some(amphipod);

                    (burrow, cost)
                })
            })
        })
    }

    fn descending_moves(&self) -> impl Iterator<Item = (Burrow, usize)> + '_
    {
        self.hallway.iter().enumerate().filter_map(|(source, amphipod)|
        {
            amphipod.and_then(|amphipod|
            {
                let ri = amphipod.index();
                self.rooms[ri].iter().all(|&a| a == amphipod).then(||
                {
                    let hi = 2 * (ri + 1);
                    if hi < source { &self.hallway[hi .. source] } else { &self.hallway[source+1 ..= hi] }
                })
                .and_then(|path|
                {
                    path.iter().all(Option::is_none).then(||
                    {
                        let mut burrow         = self.clone();
                        let room               = &mut burrow.rooms[ri];
                        let cost               = amphipod.energy() * (path.len() + burrow.depth - room.len());
                        burrow.hallway[source] = None;
                        room.push(amphipod);

                        (burrow, cost)
                    })
                })
            })
        })
    }
}
