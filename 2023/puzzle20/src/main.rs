use std::collections::{ HashMap, VecDeque };

fn main()
{
    let mut graph = include_str!("../input.txt").lines().map(Module::parse).collect::<HashMap<&str, Module<&str>>>();
    Module::reset(&mut graph);

    let mut highs = 0;
    let mut lows  = 0;
    let mut queue = VecDeque::new();

    for _ in 0 .. 1000
    {
        queue.push_back(("button", "broadcaster", false));
        while let Some((source, dest, in_pulse)) = queue.pop_front()
        {
            if in_pulse { highs += 1 } else { lows += 1 }
            if let Some(module) = graph.get_mut(dest)
            {
                queue.extend(module.update(source, in_pulse).map(|(out, out_pulse)| (dest, out, out_pulse)));
            }
        }
    }
    println!("{}", highs * lows);
}

struct Module<Id>
{
    state:   State<Id>,
    outputs: Vec<Id>
}

enum State<Id>
{
    Broadcaster,
    FlipFlop(bool),
    Conjunction(HashMap<Id, bool>)
}

impl Module<&str>
{
    fn parse(s : &str) -> (&str, Module<&str>)
    {
        let (id, outputs) = s.split_once(" -> ").unwrap();

        let state = match id.as_bytes()[0]
        {
            b'b' => State::Broadcaster,
            b'%' => State::FlipFlop(false),
            b'&' => State::Conjunction(HashMap::new()),
            _    => unreachable!()
        };

        let id      = if let State::Broadcaster = state { id } else { &id[1..] };
        let outputs = outputs.split(", ").collect();

        (id, Module { state, outputs })
    }
}

impl<Id : Copy + Eq + std::hash::Hash> Module<Id>
{
    fn reset(modules : &mut HashMap<Id, Module<Id>>)
    {
        let mut buffer = HashMap::new();

        for (&id, module) in modules.iter()
        {
            for &output in module.outputs.iter()
            {
                buffer.entry(output).or_insert_with(Vec::new).push(id);
            }
        }

        for (id, inputs) in buffer.into_iter()
        {
            if let Some(module) = modules.get_mut(&id)
            {
                if let State::Conjunction(mem) = &mut module.state
                {
                    inputs.into_iter().for_each(|input| { mem.insert(input, false); })
                }
            }
        }
    }

    fn update(&mut self, input : Id, in_pulse : bool) -> impl Iterator<Item = (Id, bool)> + '_
    {
        let out_pulse = match &mut self.state
        {
            State::Broadcaster      => Some(in_pulse),
            State::FlipFlop(s)      => (!in_pulse).then(|| { *s = !*s; *s }),
            State::Conjunction(mem) => { mem.insert(input, in_pulse); Some(!mem.iter().all(|(_, v)| *v)) }
        };

        out_pulse.into_iter().flat_map(|pulse| self.outputs.iter().map(move |&output| (output, pulse)))
    }
}
