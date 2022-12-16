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

    let mut visited = HashSet::new();
    let mut queue   = VecDeque::new();
    queue.push_back(State { valve: "AA", minutes: 30, pressure: 0, rates });

    let mut max = 0;
    while let Some(state) = queue.pop_front()
    {
        if state.minutes <= 1
        || !visited.insert((state.valve, state.pressure)) { continue }

        max = max.max(state.pressure);
        queue.extend(state.step(&tunnels));
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

struct State<'a>
{
    valve:    &'a str,
    minutes:  u32,
    pressure: u32,
    rates:    HashMap<&'a str, u32>
}

impl<'a> State<'a>
{
    fn step<'b>(&'b self, tunnels : &'a HashMap<&str, Vec<&str>>) -> impl Iterator<Item = State<'a>> + 'b
    {
        let minutes    = self.minutes - 1;
        let open_valve = self.rates.get(self.valve).into_iter().map(move |&rate|
        {
            let mut rates = self.rates.clone();
            rates.remove(self.valve);

            State
            {
                valve:    self.valve,
                minutes,
                pressure: self.pressure + rate * minutes,
                rates
            }
        });

        let tunnel = tunnels.get(self.valve).into_iter().flat_map(move |ts| ts.iter().map(move |valve| State
        {
            valve,
            minutes,
            pressure:  self.pressure,
            rates:     self.rates.clone()
        }));

        open_valve.chain(tunnel)
    }
}
