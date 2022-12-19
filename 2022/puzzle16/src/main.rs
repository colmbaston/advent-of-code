use std::{ cmp::Ordering, collections::{ HashMap, hash_map::Entry }};

fn main()
{
    let mut valves  = HashMap::new();
    let mut tunnels = HashMap::new();
    for (valve, rate, ts) in include_str!("../input.txt").lines().filter_map(parse_valve)
    {
        if rate > 0 { valves.insert(valve, rate); }
        tunnels.insert(valve, ts);
    }

    let vertices      = tunnels.keys().copied();
    let edges         = tunnels.iter().flat_map(|(&v, ts)| ts.iter().map(move |&t| ((v, t), 1)));
    let mut distances = aoc::pathfinding::floyd_warshall(vertices, edges);
    drop(tunnels);

    for dist in distances.values_mut() { *dist += 1 }
    distances.retain(|(source, dest), _| (valves.contains_key(source) || source == &"AA")
                                      &&  valves.contains_key(dest));

    let mut queue         = Vec::new();
    let mut cache         = HashMap::new();
    let mut valves_sorted = valves.keys().copied().collect::<Vec<&str>>();
    valves_sorted.sort_unstable();

    {
        let state = State { valve: "AA", minutes: 30, pressure: 0, closed: valves.clone() };
        dfs(state, &valves_sorted, &distances, &mut queue, &mut cache);

        let mut max = 0;
        for (_, pressure) in cache.drain()
        {
            max = max.max(pressure);
        }
        println!("{max}");
    }

    {
        let state = State { valve: "AA", minutes: 26, pressure: 0, closed: valves };
        dfs(state, &valves_sorted, &distances, &mut queue, &mut cache);

        let mut max = 0;
        for (i, (open_a, pressure_a)) in cache.iter().enumerate()
        {
            'middle: for (open_b, pressure_b) in cache.iter().skip(i+1)
            {
                let mut open_a = open_a.iter();
                let mut peek_a = open_a.next();

                let mut open_b = open_b.iter();
                let mut peek_b = open_b.next();

                while let Some((a, b)) = peek_a.and_then(|a| peek_b.map(|b| (a, b)))
                {
                    match a.cmp(b)
                    {
                        Ordering::Less    => peek_a = open_a.next(),
                        Ordering::Equal   => continue 'middle,
                        Ordering::Greater => peek_b = open_b.next()
                    };
                }

                max = max.max(pressure_a + pressure_b)
            }
        }
        println!("{max}");
    }
}

fn dfs<'a>(init      : State<'a>,
           valves    : &[&'a str],
           distances : &HashMap<(&'a str, &'a str), u32>,
           queue     : &mut Vec<State<'a>>,
           cache     : &mut HashMap<Vec<&'a str>, u32>)
{
    queue.push(init);
    while let Some(state) = queue.pop()
    {
        match cache.entry(valves.iter().copied()
                                .filter(|v| !state.closed.contains_key(v))
                                .collect::<Vec<&str>>())
        {
            Entry::Vacant(entry) =>
            {
                entry.insert(state.pressure);
                queue.extend(state.adjacents(distances))
            }
            Entry::Occupied(mut entry) =>
            {
                let prev = entry.get_mut();
                if *prev < state.pressure
                {
                    *prev = state.pressure;
                    queue.extend(state.adjacents(distances))
                }
            }
        }
    }
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
    closed:   HashMap<&'a str, u32>
}

impl<'a> State<'a>
{
    fn adjacents<'b>(&'b self, distances : &'b HashMap<(&str, &str), u32>) -> impl Iterator<Item = State<'a>> + 'b
    {
        self.closed.keys().filter_map(|&dest|
        {
            let &distance   = distances.get(&(self.valve, dest))?;
            let minutes     = self.minutes.checked_sub(distance)?;

            let mut state   = self.clone();
            state.valve     = dest;
            state.minutes   = minutes;
            state.pressure += minutes * state.closed.remove(dest)?;
            Some(state)
        })
    }
}
