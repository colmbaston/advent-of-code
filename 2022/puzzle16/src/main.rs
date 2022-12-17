use std::collections::{ HashMap, HashSet, VecDeque };

fn main()
{
    let mut rates   = HashMap::new();
    let mut tunnels = HashMap::new();
    for (valve, rate, ts) in include_str!("../input.txt").lines().filter_map(parse_valve)
    {
        if rate > 0 { rates.insert(valve, rate); }
        tunnels.insert(valve, ts);
    }

    let vertices      = tunnels.keys().copied();
    let edges         = tunnels.iter().flat_map(|(&v, ts)| ts.iter().map(move |&t| ((v, t), 1)));
    let mut distances = aoc::pathfinding::floyd_warshall(vertices, edges);
    drop(tunnels);

    for dist in distances.values_mut() { *dist += 1 }
    distances.retain(|(source, dest), _| (rates.contains_key(source) || source == &"AA")
                                      &&  rates.contains_key(dest));

    let mut visited = HashSet::new();
    let mut queue   = VecDeque::new();
    queue.push_back(State { valve: "AA", minutes: 30, pressure: 0, rates });

    let mut max = 0;
    while let Some(state) = queue.pop_front()
    {
        if !visited.insert((state.valve, state.pressure)) { continue }

        max = max.max(state.pressure);
        queue.extend(state.adjacents(&distances));
    }
    println!("{max}");
}

fn parse_valve(s : &str) -> Option<(&str, u32, Vec<&str>)>
{
    let s          = s.strip_prefix("Valve ")?;
    let (valve, s) = s.split_at(s.find(' ')?);
    let s          = s.strip_prefix(" has flow rate=")?;
    let (rate,  s) = s.split_at(s.find(';')?);
    let s          = s.strip_prefix("; tunnels lead to valves ").or(
                     s.strip_prefix("; tunnel leads to valve "))?;

    Some((valve, rate.parse().ok()?, s.split(", ").collect()))
}

#[derive(Clone)]
struct State<'a>
{
    valve:    &'a str,
    minutes:  u32,
    pressure: u32,
    rates:    HashMap<&'a str, u32>
}

impl<'a> State<'a>
{
    fn adjacents<'b>(&'b self, distances : &'b HashMap<(&str, &str), u32>) -> impl Iterator<Item = State<'a>> + 'b
    {
        self.rates.keys().filter_map(|&dest|
        {
            let &distance   = distances.get(&(self.valve, dest))?;
            let minutes     = self.minutes.checked_sub(distance)?;

            let mut state   = self.clone();
            state.valve     = dest;
            state.minutes   = minutes;
            state.pressure += minutes * state.rates.remove(dest)?;
            Some(state)
        })
    }
}
