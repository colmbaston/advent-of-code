use std::collections::HashSet;

mod inventory;
use inventory::{ Inventory, Blueprint, Resource };

fn main()
{
    let blueprints = include_str!("../input.txt").lines()
                                                 .map(|l| Blueprint::parse(l).unwrap())
                                                 .collect::<Vec<Blueprint>>();

    let mut queue   = Vec::new();
    let mut buffer  = Vec::new();
    let mut visited = HashSet::new();

    println!("{}", blueprints.iter().map(|b| b.id *  explore(b, 24, &mut queue, &mut buffer, &mut visited)).sum::<u16>());
    println!("{}", blueprints.iter().take(3).map(|b| explore(b, 32, &mut queue, &mut buffer, &mut visited)).product::<u16>());
}

fn explore(blueprint : &Blueprint, minutes : u16, queue : &mut Vec<State>, buffer : &mut Vec<State>, visited : &mut HashSet<State>) -> u16
{
    queue.push(State::init(minutes));
    visited.clear();

    let cost_max = Resource::enumerate().fold(Inventory::EMPTY, |acc, resource| acc.max(&blueprint[resource]));

    let mut geode_max = 0;
    while let Some(state) = queue.pop()
    {
        if state.minutes == 0
        {
            geode_max = geode_max.max(state.resources[Resource::Geode]);
            continue
        }

        if !visited.insert(state.clone()) { continue }

        state.step(blueprint, buffer);
        queue.extend(buffer.drain(..)
                           .map(|mut next| { next.dump_excess(&cost_max); next })
                           .filter(|next| !next.prune(geode_max, &cost_max)));
    }
    geode_max
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct State
{
    minutes:   u16,
    resources: Inventory,
    robots:    Inventory
}

impl State
{
    fn init(minutes : u16) -> State
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
        buffer.extend(Resource::enumerate().filter_map(|resource|
        {
            let resources  = self.resources.checked_sub(&blueprint[resource])?;
            let mut robots = self.robots.clone();
            robots[resource] += 1;
            Some(State { minutes: self.minutes, resources, robots })
        }));

        for state in buffer.iter_mut()
        {
            state.minutes -= 1;
            for resource in Resource::enumerate()
            {
                state.resources[resource] += self.robots[resource];
            }
        }
    }

    fn dump_excess(&mut self, cost_max : &Inventory)
    {
        for resource in Resource::enumerate().take(3)
        {
            self.resources[resource] = self.resources[resource].min(self.minutes * cost_max[resource]);
        }
    }

    fn prune(&self, geode_max : u16, cost_max : &Inventory) -> bool
    {
        self.geode_upper_bound() <= geode_max ||
        self.redundant_robots(cost_max)
    }

    fn geode_upper_bound(&self) -> u16
    {
        self.resources[Resource::Geode] +
        self.minutes * self.robots[Resource::Geode] +
        self.minutes * self.minutes.saturating_sub(1) / 2
    }

    fn redundant_robots(&self, cost_max : &Inventory) -> bool
    {
        Resource::enumerate().take(3).any(|resource|
        {
            self.robots[resource] > cost_max[resource]
        })
    }
}
