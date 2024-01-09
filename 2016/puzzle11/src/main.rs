fn main()
{
    let mut state = State::parse(include_str!("../input.txt"));
    println!("{}", aoc::pathfinding::bfs(std::iter::once(state.clone()),
                                         State::target,
                                         State::adjacent).unwrap_or(0usize));

    state.floors[0].generators.extend(["elerium", "dilithium"]);
    state.floors[0].microchips.extend(["elerium", "dilithium"]);
    println!("{}", aoc::pathfinding::bfs(std::iter::once(state),
                                         State::target,
                                         State::adjacent).unwrap_or(0usize));
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct State<T>
{
    elevator: usize,
    floors:   Vec<Floor<T>>
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Floor<T>
{
    generators: Vec<T>,
    microchips: Vec<T>
}

impl State<&str>
{
    fn parse(s : &str) -> State<&str>
    {
        let floors = s.lines().map(Floor::parse).collect();
        State { elevator: 0, floors }
    }
}

impl Floor<&str>
{
    fn parse(s : &str) -> Floor<&str>
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

                match b
                {
                    "generator"            => generators.push(a),
                    "compatible microchip" => microchips.push(a),
                    _                      => unreachable!()
                }
            }
        }
        Floor { generators, microchips }
    }
}

impl<T> State<T>
{
    fn target(&self) -> bool
    {
        self.floors.split_last()
                   .map(|(_, rest)| rest.iter().all(Floor::is_empty))
                   .unwrap_or(true)
    }
}

impl<T : Clone + Ord> State<T>
{
    fn adjacent(&self) -> impl Iterator<Item = State<T>>
    {
        let mut adjs = Vec::new();
        let mut next = Vec::new();
        if self.elevator   > 0                 { next.push(self.elevator-1) }
        if self.elevator+1 < self.floors.len() { next.push(self.elevator+1) }

        for (p1, current) in self.floors[self.elevator].remove_one()
        {
            if current.valid()
            {
                for &i in next.iter()
                {
                    let mut next = self.floors[i].clone();
                    next.insert(p1.clone());
                    if next.valid()
                    {
                        let mut state               = self.clone();
                        state.elevator              = i;
                        state.floors[self.elevator] = current.clone();
                        state.floors[i]             = next;
                        adjs.push(state)
                    }
                }
            }
        }

        for (p1, p2, current) in self.floors[self.elevator].remove_two()
        {
            if current.valid()
            {
                for &i in next.iter()
                {
                    let mut next = self.floors[i].clone();
                    next.insert(p1.clone());
                    next.insert(p2.clone());
                    if next.valid()
                    {
                        let mut state               = self.clone();
                        state.elevator              = i;
                        state.floors[self.elevator] = current.clone();
                        state.floors[i]             = next;
                        adjs.push(state)
                    }
                }
            }
        }

        adjs.iter_mut().for_each(State::normalise);
        adjs.into_iter()
    }
}

impl<T : Eq> Floor<T>
{
    fn valid(&self) -> bool
    {
        self.generators.is_empty() ||
        self.microchips.iter().all(|k| self.generators.contains(k))
    }
}

impl<T : Ord> State<T>
{
    fn normalise(&mut self)
    {
        self.floors.iter_mut().for_each(Floor::normalise)
    }
}

impl<T> Floor<T>
{
    fn is_empty(&self) -> bool
    {
        self.generators.is_empty() && self.microchips.is_empty()
    }

    fn len(&self) -> usize
    {
        self.generators.len() + self.microchips.len()
    }

    fn insert(&mut self, (gen, element) : (bool, T))
    {
        if gen { self.generators.push(element) }
        else   { self.microchips.push(element) }
    }

    fn remove(&mut self, i : usize) -> (bool, T)
    {
        let gen = i < self.generators.len();

        (gen, if gen { self.generators.swap_remove(i)                         }
              else   { self.microchips.swap_remove(i - self.generators.len()) })
    }
}

impl<T : Clone> Floor<T>
{
    fn remove_one(&self) -> impl Iterator<Item = ((bool, T), Floor<T>)> + '_
    {
        (0 .. self.len()).map(|i|
        {
            let mut floor = self.clone();
            (floor.remove(i), floor)
        })
    }

    fn remove_two(&self) -> impl Iterator<Item = ((bool, T), (bool, T), Floor<T>)> + '_
    {
        (1 .. self.len()).flat_map(move |i| (0 .. i).map(move |j|
        {
            let mut floor = self.clone();
            (floor.remove(i), floor.remove(j), floor)
        }))
    }
}

impl<T : Ord> Floor<T>
{
    fn normalise(&mut self)
    {
        self.generators.sort_unstable();
        self.microchips.sort_unstable();
    }
}
