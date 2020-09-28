use std::collections::{ HashSet, HashMap, BinaryHeap };

fn main()
{
    // parse input from statically-included file
    let (_, cave) = parse_cave(include_str!("../input.txt")).unwrap();
    let (tx, ty)  = cave.target;

    // a cache of the cave's erosion levels
    let mut cache = HashMap::new();

    // force the caching of the rectangle from (0, 0) to the target
    // (0, 0) and the target won't be present in the cache, but they don't change the result
    erosion_level(&mut cache, (tx, ty - 1), &cave);
    erosion_level(&mut cache, (tx - 1, ty), &cave);

    // part one: sum the risk levels of the rectangle from (0, 0) to the target
    println!("{}", cache.values().fold(0, |a, x| a + region_type(*x) as u32));

    // part two: use A* search to find the number of minutes to reach target
    println!("{}", astar(&mut cache, &cave));
}

struct Cave
{
    depth  :  u32,
    target : (i32, i32)
}

fn parse_cave(s : &str) -> nom::IResult<&str, Cave>
{
    use nom::character::complete::digit1;

    let (s, depth) = digit1(&s[7..])?;
    let (s, tx)    = digit1(&s[9..])?;
    let (s, ty)    = digit1(&s[1..])?;

    Ok((s, Cave
    {
        depth:  depth.parse().unwrap(),
        target: (tx.parse().unwrap(), ty.parse().unwrap())
    }))
}

type Cache = HashMap<(i32, i32), u32>;

fn erosion_level(cache : &mut Cache, position : (i32, i32), cave : &Cave) -> u32
{
    match cache.get(&position)
    {
        // if the erosion level is already present in the cache, return it
        Some(&erosion) => erosion,
        None           =>
        {
            // otherwise compute it
            let geologic = match position
            {
                t if t == cave.target => 0,
                (x, 0)                => x as u32 * 16807,
                (0, y)                => y as u32 * 48271,
                (x, y)                => erosion_level(cache, (x-1, y), cave)
                                       * erosion_level(cache, (x, y-1), cave)
            };
            let erosion = (geologic + cave.depth) % 20183;

            // and insert it into the cache before returning it
            cache.insert(position, erosion);
            erosion
        }
    }
}

enum Region
{
    Rocky  = 0,
    Wet    = 1,
    Narrow = 2
}

#[inline]
fn region_type(erosion : u32) -> Region
{
    match erosion % 3
    {
        0 => Region::Rocky,
        1 => Region::Wet,
        2 => Region::Narrow,
        _ => unreachable!()
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    fn moves(&self, cache : &mut Cache, cave : &Cave) -> Vec<(State, u32)>
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
                match (region_type(erosion_level(cache, position, cave)), &self.tool)
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
        let tool = match (region_type(erosion_level(cache, self.position, cave)), &self.tool)
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
// use Manhattan distance to the target as the A* heuristic
fn manhattan((x1, y1) : (i32, i32), (x2, y2) : (i32, i32)) -> u32
{
    ((x1 - x2).abs() + (y1 - y2).abs()) as u32
}

fn astar(cache : &mut Cache, cave : &Cave) -> u32
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
        if state.position == cave.target && state.tool == Tool::Torch { return steps }

        queue.extend(state.moves(cache, cave).into_iter().filter_map(|(state, k)|
        {
            if visited.contains(&state)
            {
                None
            }
            else
            {
                let steps = steps + k;
                Some((Reverse(steps + manhattan(state.position, cave.target)), steps, state))
            }
        }));

        visited.insert(state);
    }

    panic!("exhausted A* search without finding the target");
}
