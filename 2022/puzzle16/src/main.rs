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

    let mut queue     = VecDeque::new();
    let mut visited   = HashSet::new();
    let mut adjacents = Vec::new();
    let mut buffer    = Vec::new();

    let start_one = State { valves: vec!["AA"],       minutes: 30, pressure: 0, rates: rates.clone() };
    let start_two = State { valves: vec!["AA", "AA"], minutes: 26, pressure: 0, rates                };
    for start in [start_one, start_two]
    {
        queue.clear();
        queue.push_back(start);
        visited.clear();

        let mut max = 0;
        while let Some(state) = queue.pop_front()
        {
            if !visited.insert((state.valves.clone(), state.pressure)) { continue }

            max = max.max(state.pressure);
            state.adjacents(&tunnels, &mut adjacents, &mut buffer);
            queue.extend(adjacents.drain(..));
        }
        println!("{max}");
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
    valves:   Vec<&'a str>,
    minutes:  u32,
    pressure: u32,
    rates:    HashMap<&'a str, u32>
}

impl<'a> State<'a>
{
    fn adjacents<'b>(&'b self, tunnels : &'a HashMap<&str, Vec<&str>>, result : &'b mut Vec<State<'a>>, buffer : &'b mut Vec<State<'a>>)
    {
        let mut state = self.clone();
        state.minutes -= 1;
        if state.minutes <= 1 { return }

        result.clear();
        result.push(state);
        for (ix, valve) in self.valves.iter().enumerate()
        {
            buffer.clear();
            buffer.extend(result.iter().flat_map(|state|
            {
                let open_valve = state.rates.get(valve).into_iter().map(|&rate|
                {
                    let mut state = state.clone();
                    state.pressure += rate * state.minutes;
                    state.rates.remove(valve);
                    state
                });

                let move_valve = tunnels.get(valve).into_iter().flat_map(|ts| ts.iter().map(|valve|
                {
                    let mut state = state.clone();
                    state.valves[ix] = valve;
                    state
                }));

                open_valve.chain(move_valve)
            }));
            std::mem::swap(result, buffer)
        }
    }
}
