#![feature(iter_next_chunk)]
use std::collections::HashMap;
use aoc::direction::Direction;

fn main()
{
    let mut grid = Tile::parse_grid(include_str!("../input.txt"));
    let width    = grid[0].len();
    let height   = grid.len();

    let mut graph_one = HashMap::new();
    let mut graph_two = HashMap::new();
    for (row, y) in grid.iter().zip(0..)
    {
        for (tile, x) in row.iter().zip(0..).filter(|(tile, _)| tile.passable())
        {
            let pos  = (x, y);
            let adjs = adjacents(pos, &grid).map(|pos| (pos, 1)).collect::<HashMap<Pos, u32>>();

            match tile
            {
                Tile::Path =>
                {
                    graph_one.insert(pos, adjs.clone());
                    graph_two.insert(pos, adjs);
                },
                Tile::Slope(dir) =>
                {
                    graph_one.insert(pos, std::iter::once((dir.step(pos), 1)).collect());
                    graph_two.insert(pos, adjs);
                },
                Tile::Forest => unreachable!(),
            };
        }
    }
    contract(&mut graph_one);
    contract(&mut graph_two);

    let start  = (1, 0);
    let target = (width as u8 - 2, height as u8 - 1);
    println!("{}", dfs(start, target, 0, &graph_one, &mut grid));
    println!("{}", dfs(start, target, 0, &graph_two, &mut grid));
}

type Grid = Vec<Vec<Tile>>;

#[derive(Copy, Clone)]
enum Tile { Path, Forest, Slope(Direction) }

impl Tile
{
    fn parse_grid(s : &str) -> Grid
    {
        s.lines().map(|l| l.bytes().map(|b| match b
        {
            b'.' => Tile::Path,
            b'#' => Tile::Forest,
            b'^' => Tile::Slope(Direction::North),
            b'>' => Tile::Slope(Direction::East),
            b'v' => Tile::Slope(Direction::South),
            b'<' => Tile::Slope(Direction::West),
            _    => unreachable!()
        })
        .collect()).collect()
    }

    fn passable(self) -> bool
    {
        matches!(self, Tile::Path | Tile::Slope(_))
    }
}

type Pos   = (u8, u8);
type Graph = HashMap<Pos, HashMap<Pos, u32>>;

fn adjacents(pos : Pos, grid : &Grid) -> impl Iterator<Item = Pos> + '_
{
    Direction::ELEMS.into_iter()
                    .filter_map(move |dir| dir.checked_step(pos))
                    .filter(|&(x, y)| grid.get(y as usize)
                                          .and_then(|row| row.get(x as usize))
                                          .filter(|tile| tile.passable())
                                          .is_some())
}

fn contract(graph : &mut Graph)
{
    let buffer = graph.iter()
                      .filter_map(|(&pos, edges)| (edges.len() == 2).then_some(pos))
                      .collect::<Vec<Pos>>();

    for pos in buffer.into_iter()
    {
        let [(&p1, &c1), (&p2, &c2)] = graph[&pos].iter().next_chunk().unwrap();
        graph.remove(&pos);
        graph.entry(p1).and_modify(|edges| { edges.remove(&pos); edges.insert(p2, c1+c2); });
        graph.entry(p2).and_modify(|edges| { edges.remove(&pos); edges.insert(p1, c1+c2); });
    }
}

fn dfs(pos@(x, y) : Pos, target : Pos, steps : u32, graph : &Graph, grid : &mut Grid) -> u32
{
    if pos == target { return steps }

    let tile = std::mem::replace(&mut grid[y as usize][x as usize], Tile::Forest);

    #[allow(clippy::filter_map_bool_then)]
    let max = graph.get(&pos).unwrap().iter()
                   .filter_map(|(&next@(x, y), cost)| grid[y as usize]
                                                          [x as usize].passable()
                                                                      .then(|| dfs(next, target, steps+cost, graph, grid)))
                   .max().unwrap_or(0);

    grid[y as usize][x as usize] = tile;
    max
}
