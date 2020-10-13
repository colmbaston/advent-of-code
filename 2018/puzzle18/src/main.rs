use std::collections::HashMap;

fn main()
{
    let mut area    = parse(include_str!("../input.txt"));

    // part 1: what is the resource value after 10 minutes?
    for _ in 0 .. 10 { area = generation(&area) }
    println!("{}", resource_value(area.iter().flat_map(|v| v.iter())));

    // part 2: what is the resouce values after 1_000_000_000 minutes?
    // detect the generation of the first repeated state
    let mut gen     = 10;
    let mut visited = HashMap::new();
    let repeat = loop
    {
        if let Some(i) = visited.get(&area)
        {
            break i
        }

        let new = generation(&area);
        visited.insert(area, gen);
        area = new;
        gen += 1;
    };

    // simulate up to 1_000_000 mod the cycle length
    let cycle = gen - repeat;
    for _ in 0 .. (1_000_000_000 - repeat) % cycle
    {
        area = generation(&area);
    }
    println!("{}", resource_value(area.iter().flat_map(|v| v.iter())));
}

type Area = Vec<Vec<Tile>>;

#[derive(Clone, PartialEq, Eq, Hash)]
enum Tile
{
    Open,
    Trees,
    Lumber
}

fn parse(s : &str) -> Area
{
    let mut area = Vec::with_capacity(50);
    for l in s.lines()
    {
        area.push(l.bytes().map(|b| match b
        {
            b'.' => Tile::Open,
            b'|' => Tile::Trees,
            b'#' => Tile::Lumber,
            _    => unreachable!()
        })
        .collect());
    }
    area
}

fn generation(area : &Area) -> Area
{
    let mut new = Vec::with_capacity(area.len());
    for y in 0 .. area.len()
    {
        let row_len = area[0].len();
        let mut row = Vec::with_capacity(row_len);
        for x in 0 .. row_len
        {
            let (_, trees, lumber) = count_tiles(adjacents((x, y), area));
            row.push(match area[y][x]
            {
                Tile::Open   => if trees  >= 3                { Tile::Trees  } else { Tile::Open  }
                Tile::Trees  => if lumber >= 3                { Tile::Lumber } else { Tile::Trees }
                Tile::Lumber => if trees  >= 1 && lumber >= 1 { Tile::Lumber } else { Tile::Open  }
            });
        }
        new.push(row);
    }
    new
}

fn adjacents((x, y) : (usize, usize), area : &Area) -> impl Iterator<Item = &Tile>
{
    let left  = x.wrapping_sub(1);
    let above = y.wrapping_sub(1);
    let adjs  = vec![(left, above), (x, above), (x+1, above),
                     (left, y    ),             (x+1, y    ),
                     (left, y+1  ), (x, y+1  ), (x+1, y+1  )];

    adjs.into_iter().filter_map(move |(x, y)| area.get(y).and_then(|v| v.get(x)))
}

fn count_tiles<'a>(tiles : impl Iterator<Item = &'a Tile>) -> (u32, u32, u32)
{
    tiles.fold((0, 0, 0), |(open, trees, lumber), t| match t
    {
        Tile::Open   => (open+1, trees,   lumber  ),
        Tile::Trees  => (open,   trees+1, lumber  ),
        Tile::Lumber => (open,   trees,   lumber+1)
    })
}

fn resource_value<'a>(tiles : impl Iterator<Item = &'a Tile>) -> u32
{
    let (_, trees, lumber) = count_tiles(tiles);
    trees * lumber
}
