use std::collections::HashSet;

mod resource;
use resource::{ Resource, Inventory, Blueprint };


fn main()
{
    let blueprints = include_str!("../input.txt").lines()
                                                 .map(|l| Blueprint::parse(l).unwrap())
                                                 .collect::<Vec<Blueprint>>();

    let mut queue   = Vec::new();
    let mut buffer  = Vec::new();
    let mut visited = HashSet::new();

    let mut sum = 0;
    for blueprint in blueprints.iter()
    {
        queue.push(State::init(24));
        visited.clear();

        let mut geodes = 0;
        while let Some(state) = queue.pop()
        {
            if state.minutes == 0
            {
                geodes = geodes.max(state.resources[Resource::Geode]);
                continue
            }

            if !visited.insert(state.clone()) { continue }
            state.step(blueprint, &mut buffer);
            queue.extend(buffer.drain(..).filter(|next| !next.prune(geodes, blueprint, &state)));
        }

        let quality = geodes * blueprint.id;
        sum += quality;
        println!("blueprint {}: cracked {geodes} geodes, quality {quality}", blueprint.id);
    }
    println!("{sum}");
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct State
{
    minutes:   u8,
    resources: Inventory,
    robots:    Inventory
}

impl State
{
    fn init(minutes : u8) -> State
    {
        let mut state = State
        {
            minutes,
            resources: Inventory::EMPTY,
            robots:    Inventory::EMPTY
        };

        state.robots[Resource::Ore] = 1;
        state
    }

    fn step(&self, blueprint : &Blueprint, buffer : &mut Vec<State>)
    {
        buffer.push(self.clone());
        buffer.extend(Resource::iter().filter_map(|resource|
        {
            let resources  = self.resources.checked_sub(&blueprint[resource])?;
            let mut robots = self.robots.clone();
            robots[resource] += 1;
            Some(State { minutes: self.minutes, resources, robots })
        }));

        for state in buffer.iter_mut()
        {
            state.minutes -= 1;
            for resource in Resource::iter()
            {
                state.resources[resource] += self.robots[resource];
            }
        }
    }

    fn prune(&self, _geodes : u16, _blueprint : &Blueprint, _prev : &State) -> bool
    {
        false
    }
}
