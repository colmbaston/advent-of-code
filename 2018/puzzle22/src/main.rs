use std::collections::{ HashSet, HashMap, BinaryHeap };

const DEPTH  :  u32       =  10914;
const TARGET : (i32, i32) = (9, 739);

fn main()
{
    // a cache of the cave's geologic indices
    let mut cache = HashMap::new();

    // force the caching of the rectangle from (0, 0) to TARGET
    geologic_index(&mut cache, TARGET);
    geologic_index(&mut cache, (TARGET.0, TARGET.1 - 1));
    geologic_index(&mut cache, (TARGET.0 - 1, TARGET.1));

    // part one: sum the risk levels of the rectangle from (0, 0) to TARGET
    println!("{}", cache.values().fold(0, |a, x| a + region_type(*x) as u32));

    // part two: use A* search to find the number of minutes to reach TARGET
    println!("{}", astar(&mut cache));
}

type Cache = HashMap<(i32, i32), u32>;

fn geologic_index(cache : &mut Cache, position : (i32, i32)) -> u32
{
    match cache.get(&position)
    {
        // if the geologic index is already present in the cache, return it
        Some(index) => *index,
        None        =>
        {
            // otherwise compute it
            let index = match position
            {
                TARGET => 0,
                (x, 0) => x as u32 * 16807,
                (0, y) => y as u32 * 48271,
                (x, y) => erosion_level(geologic_index(cache, (x-1, y)))
                        * erosion_level(geologic_index(cache, (x, y-1)))
            };

            // and insert it into the cache before returning it
            cache.insert(position, index);
            index
        }
    }
}

#[inline]
fn erosion_level(geologic_index : u32) -> u32
{
    (geologic_index + DEPTH) % 20183
}

enum Region
{
    Rocky  = 0,
    Wet    = 1,
    Narrow = 2
}

#[inline]
fn region_type(geologic_index : u32) -> Region
{
    match erosion_level(geologic_index) % 3
    {
        0 => Region::Rocky,
        1 => Region::Wet,
        _ => Region::Narrow
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct State
{
    position: (i32, i32),
    tool:     Tool
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Tool
{
    Torch,
    ClimbingGear,
    Neither
}

impl State
{
    // generate possible moves from one state to the next, along with how many minutes each move takes
    fn moves(&self, cache : &mut Cache) -> Vec<(State, u32)>
    {
        let (x, y) = self.position;

        // you can move to an adjacent region in one minute
        let adjacents  = vec![(x+1, y), (x, y+1), (x-1, y), (x, y-1)];
        let mut result = adjacents.into_iter().filter_map(|position|
        {
            // you cannot move into the negative coordinates
            if position.0 < 0 || position.1 < 0
            {
                None
            }
            else
            {
                // you can only move into the adjacent region with an appropriate tool equipped
                match (region_type(geologic_index(cache, position)), &self.tool)
                {
                    (Region::Rocky,  Tool::Neither)      => None,
                    (Region::Wet,    Tool::Torch)        => None,
                    (Region::Narrow, Tool::ClimbingGear) => None,
                    _                                    => Some((State { position, tool: self.tool.clone() }, 1))
                }
            }
        })
        .collect::<Vec<_>>();

        // you can switch to the other tool available for the current region in seven minutes
        let tool = match (region_type(geologic_index(cache, self.position)), &self.tool)
        {
            (Region::Rocky,  Tool::Torch)        => Tool::ClimbingGear,
            (Region::Rocky,  Tool::ClimbingGear) => Tool::Torch,
            (Region::Wet,    Tool::ClimbingGear) => Tool::Neither,
            (Region::Wet,    Tool::Neither)      => Tool::ClimbingGear,
            (Region::Narrow, Tool::Torch)        => Tool::Neither,
            (Region::Narrow, Tool::Neither)      => Tool::Torch,
            _                                    => panic!("impossible combination of region type and tool")
        };
        result.push((State { position: self.position, tool }, 7));

        result
    }
}

#[inline]
// use Manhattan distance to TARGET as the A* heuristic
fn manhattan(&(x, y) : &(i32, i32)) -> u32
{
    ((x - TARGET.0).abs() + (y - TARGET.1).abs()) as u32
}

fn astar(cache : &mut Cache) -> u32
{
    use std::cmp::Reverse;

    // initialise the visited state and priority queue
    let mut visited = HashSet::new();
    let mut queue   = BinaryHeap::new();
    queue.push((Reverse(0), 0, State { position: (0, 0), tool: Tool::Torch }));

    // explore states from the queue, prioritising those which minimise the heuristic
    while let Some((_, steps, state)) = queue.pop()
    {
        if visited.contains(&state) { continue }
        if let State { position: TARGET, tool: Tool::Torch } = state { return steps }

        queue.extend(state.moves(cache).into_iter().filter_map(|(state, k)|
        {
            if visited.contains(&state)
            {
                None
            }
            else
            {
                Some((Reverse(steps + k + manhattan(&state.position)), steps + k, state))
            }
        }));

        visited.insert(state);
    }

    panic!("exhausted A* search without finding TARGET");
}
