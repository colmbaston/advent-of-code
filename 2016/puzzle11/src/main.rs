use std::collections::HashMap;

fn main()
{
    let mut cache = HashMap::new();
    let mut state = State::parse(include_str!("../input.txt"), &mut cache);
    println!("{}", aoc::pathfinding::bfs(std::iter::once(state.clone()),
                                         State::target,
                                         |s| s.adjacent().into_iter()).unwrap_or(0_usize));

    let elements = cache.len() as u8;
    state.floors[0].generators.extend((elements ..).take(2));
    state.floors[0].microchips.extend((elements ..).take(2));
    println!("{}", aoc::pathfinding::bfs(std::iter::once(state),
                                         State::target,
                                         |s| s.adjacent().into_iter()).unwrap_or(0_usize));
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct State
{
    elevator: usize,
    floors:   Vec<Floor>
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Floor
{
    generators: Vec<u8>,
    microchips: Vec<u8>
}

impl State
{
    fn parse<'a>(s : &'a str, cache : &mut HashMap<&'a str, u8>) -> State
    {
        let floors = s.lines().map(|l| Floor::parse(l, cache)).collect();
        State { elevator: 0, floors }
    }
}

impl Floor
{
    fn parse<'a>(s : &'a str, cache : &mut HashMap<&'a str, u8>) -> Floor
    {
        let (_, s) = s.strip_suffix('.').unwrap()
                      .split_once("contains ").unwrap();

        let mut generators = Vec::new();
        let mut microchips = Vec::new();
        for s in s.split(", ").flat_map(|s| s.split("and ")).map(|s| s.trim_end()).filter(|s| !s.is_empty())
        {
            if let Some(s) = s.strip_prefix("a ").or(s.strip_prefix("an "))
            {
                let (a, b) = s.split_once([' ', '-']).unwrap();

                let len     = cache.len() as u8;
                let element = *cache.entry(a).or_insert(len);

                match b
                {
                    "generator"            => generators.push(element),
                    "compatible microchip" => microchips.push(element),
                    _                      => unreachable!()
                }
            }
        }
        Floor { generators, microchips }
    }
}

impl State
{
    fn target(&self) -> bool
    {
        self.floors.split_last()
                   .map(|(_, rest)| rest.iter().all(Floor::is_empty))
                   .unwrap_or(true)
    }

    fn adjacent(&self) -> Vec<State>
    {
        let mut adjs = Vec::new();

        if self.elevator+1 < self.floors.len()
        {
            let mut any = false;
            for (p1, p2, current) in self.floors[self.elevator].remove_two()
            {
                if current.valid()
                {
                    let mut next = self.floors[self.elevator+1].clone();
                    next.insert(p1);
                    next.insert(p2);
                    if next.valid()
                    {
                        let mut state                 = self.clone();
                        state.elevator               += 1;
                        state.floors[self.elevator]   = current;
                        state.floors[self.elevator+1] = next;
                        adjs.push(state);

                        any = true
                    }
                }
            }

            if !any
            {
                for (p1, current) in self.floors[self.elevator].remove_one()
                {
                    if current.valid()
                    {
                        let mut next = self.floors[self.elevator+1].clone();
                        next.insert(p1);
                        if next.valid()
                        {
                            let mut state                 = self.clone();
                            state.elevator               += 1;
                            state.floors[self.elevator]   = current;
                            state.floors[self.elevator+1] = next;
                            adjs.push(state)
                        }
                    }
                }
            }
        }

        if 0 < self.elevator
        {
            let mut any = false;
            for (p1, current) in self.floors[self.elevator].remove_one()
            {
                if current.valid()
                {
                    let mut next = self.floors[self.elevator-1].clone();
                    next.insert(p1);
                    if next.valid()
                    {
                        let mut state                 = self.clone();
                        state.elevator               -= 1;
                        state.floors[self.elevator]   = current;
                        state.floors[self.elevator-1] = next;
                        adjs.push(state);

                        any = true
                    }
                }
            }

            if !any
            {
                for (p1, p2, current) in self.floors[self.elevator].remove_two()
                {
                    if current.valid()
                    {
                        let mut next = self.floors[self.elevator-1].clone();
                        next.insert(p1);
                        next.insert(p2);
                        if next.valid()
                        {
                            let mut state                 = self.clone();
                            state.elevator               -= 1;
                            state.floors[self.elevator]   = current;
                            state.floors[self.elevator-1] = next;
                            adjs.push(state)
                        }
                    }
                }
            }
        }

        adjs.iter_mut().for_each(State::normalise);
        adjs
    }

    fn normalise(&mut self)
    {
        let mut cache = HashMap::new();
        self.floors.iter_mut().for_each(|floor| floor.normalise(&mut cache))
    }
}

impl Floor
{
    fn valid(&self) -> bool
    {
        self.generators.is_empty() ||
        self.microchips.iter().all(|k| self.generators.contains(k))
    }

    fn is_empty(&self) -> bool
    {
        self.generators.is_empty() && self.microchips.is_empty()
    }

    fn len(&self) -> usize
    {
        self.generators.len() + self.microchips.len()
    }

    fn insert(&mut self, (g, element) : (bool, u8))
    {
        if g { self.generators.push(element) }
        else { self.microchips.push(element) }
    }

    fn remove(&mut self, i : usize) -> (bool, u8)
    {
        let g = i < self.generators.len();

        (g, if g { self.generators.swap_remove(i)                         }
            else { self.microchips.swap_remove(i - self.generators.len()) })
    }

    fn remove_one(&self) -> impl Iterator<Item = ((bool, u8), Floor)> + '_
    {
        (0 .. self.len()).map(|i|
        {
            let mut floor = self.clone();
            (floor.remove(i), floor)
        })
    }

    fn remove_two(&self) -> impl Iterator<Item = ((bool, u8), (bool, u8), Floor)> + '_
    {
        (1 .. self.len()).flat_map(move |i| (0 .. i).map(move |j|
        {
            let mut floor = self.clone();
            (floor.remove(i), floor.remove(j), floor)
        }))
    }

    fn normalise(&mut self, cache : &mut HashMap<u8, u8>)
    {
        self.generators.sort_unstable();
        self.microchips.sort_unstable();

        for element in self.generators.iter_mut().chain(self.microchips.iter_mut())
        {
            let len  = cache.len() as u8;
            *element = *cache.entry(*element).or_insert(len);
        }
    }
}
