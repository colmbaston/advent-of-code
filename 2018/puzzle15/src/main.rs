use std::collections::{ HashSet, HashMap, hash_map::Entry, VecDeque };

fn main()
{
    let (units, walls) = parse(include_str!("../input.txt"));

    // part 1: find the outcome of the combat
    println!("{}", combat(units.clone(), &walls, None).unwrap());

    // part 2: find the outcome of the combat where the elves' attack
    // power is the minimum it can be while having all elves survive
    for elf_attack in 4 ..
    {
        if let Some(outcome) = combat(units.clone(), &walls, Some(elf_attack))
        {
            println!("{}", outcome);
            break
        }
    }
}

#[derive(Clone)]
struct Unit
{
    team:       Team,
    hit_points: u32
}

#[derive(Clone, PartialEq, Eq)]
enum Team
{
    Goblin,
    Elf
}

fn parse(s : &str) -> (HashMap<(u32, u32), Unit>, HashSet<(u32, u32)>)
{
    let mut units = HashMap::new();
    let mut walls = HashSet::new();

    for (line, y) in s.lines().zip(0..)
    {
        for (b, x) in line.bytes().zip(0..)
        {
            match b
            {
                b'.' => (),
                b'#' => { walls.insert((x, y)); },
                _    =>
                {
                    units.insert((x, y), Unit
                    {
                        team:       if b == b'G' { Team::Goblin } else { Team::Elf },
                        hit_points: 200,
                    });
                }
            }
        }
    }

    (units, walls)
}

fn combat(mut units : HashMap<(u32, u32), Unit>, walls : &HashSet<(u32, u32)>, elf_attack : Option<u32>) -> Option<u32>
{
    let mut round          = 0;
    let mut unit_positions = Vec::new();

    loop
    {
        // units take turns in an order based on their position
        unit_positions.extend(units.keys());
        unit_positions.sort_by_key(|&(x, y)| (y, x));

        for mut current_p in unit_positions.drain(..)
        {
            if let Some(current_u) = units.remove(&current_p)
            {
                // identify all possible targets; combat ends if there aren't any
                let targets = units.iter().filter(|(_, t)| current_u.team != t.team).collect::<Vec<_>>();
                if targets.is_empty()
                {
                    // current_u has been removed from the map, so seed the fold with current_u.hit_points
                    return Some(round * units.values().fold(current_u.hit_points, |a, u| a + u.hit_points))
                }

                // identify all unique open squares that are adjacent to targets
                let open_squares = targets.iter().flat_map(|(p, _)| moves(p, &walls, &units)).collect::<HashSet<_>>();

                // only move if not already on one of the open squares
                if !open_squares.contains(&current_p)
                {
                    // use bfs to compute the unit's move
                    let sources   = moves(&current_p, &walls, &units);
                    let mut sinks = open_squares.into_iter().collect::<Vec<_>>();
                    sinks.sort_by_key(|&(x, y)| (y, x));

                    current_p = bfs(&sources, &sinks, |p| moves(p, &walls, &units)).unwrap_or(current_p);
                }

                // find the adjacent target (if any) with the lowest hit points
                let target_p = ortho(current_p).filter_map(|p|
                {
                    units.get(&p).and_then(|u| if current_u.team != u.team { Some((p, u)) } else { None })
                })
                .min_by_key(|(_, u)| u.hit_points).map(|(p, _)| p);

                // if a target was found, attack them
                if let Some(Entry::Occupied(mut e)) = target_p.map(|p| units.entry(p))
                {
                    let target       = e.get_mut();
                    let attack_power = match current_u.team
                    {
                        Team::Elf    => elf_attack.unwrap_or(3),
                        Team::Goblin => 3
                    };

                    // if the target would die, remove their
                    // entry, otherwise reduce their hit points
                    if target.hit_points <= attack_power
                    {
                        // for part two, terminate the simulation as soon as one elf dies
                        if elf_attack.is_some() && target.team == Team::Elf { return None }
                        e.remove();
                    }
                    else
                    {
                        target.hit_points -= attack_power
                    }
                }

                // remember to put the current unit back into
                // the map with a potentially updated position
                units.insert(current_p, current_u);
            }
        }

        round += 1;
    }
}

fn ortho((x, y) : (u32, u32)) -> impl Iterator<Item = (u32, u32)>
{
    vec![(x, y-1), (x-1, y), (x+1, y), (x, y+1)].into_iter()
}

fn moves(&p : &(u32, u32), walls : &HashSet<(u32, u32)>, units : &HashMap<(u32, u32), Unit>) -> Vec<(u32, u32)>
{
    ortho(p).filter(|p| !(walls.contains(p) || units.contains_key(p))).collect()
}

// a version of breadth-first search which finds the nearest of a number of souces to any of a
// number of sinks; if there is a tie, sources and sinks earlier their vectors are prioritised
fn bfs<S>(sources : &[S], sinks : &[S], moves : impl Fn(&S) -> Vec<S>) -> Option<S>
where S : Clone + Eq + std::hash::Hash
{
    let mut visited = HashSet::new();
    let mut queue   = sources.iter().map(|source| (source, source.clone(), 0)).collect::<VecDeque<_>>();

    while let Some((source, current, steps)) = queue.pop_front()
    {
        if !visited.insert(current.clone()) { continue }

        // check if the current state is a sink
        if let Some((i, _)) = sinks.iter().enumerate().find(|&(_, sink)| &current == sink)
        {
            // if it is, return the source which can reach the sink with the lowest index in the same number of steps
            let same_steps   = queue.into_iter().take_while(|&(_, _, s)| s == steps);
            let sink_indices = std::iter::once((i, source)).chain(same_steps.filter_map(|(source, current, _)|
            {
                sinks.iter().enumerate().find(|&(_, sink)| &current == sink).map(|(i, _)| (i, source))
            }));

            return sink_indices.min_by_key(|&(i, _)| i).map(|(_, s)| s.clone())
        }

        // otherwise, add the neighbours of the current node to the explore queue
        queue.extend(moves(&current).into_iter().filter_map(|next|
        {
            if visited.contains(&next) { None } else { Some((source, next, steps+1)) }
        }));
    }

    None
}
