#![feature(iter_next_chunk)]
use std::collections::{ HashMap, HashSet };
use aoc::direction::Direction;

fn main()
{
    let grid = Tile::parse_grid(include_str!("../input.txt"));

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

    let start       = (1, 0);
    let target      = (grid[0].len() as u8 - 2, grid.len() as u8 - 1);
    let mut visited = HashSet::new();
    println!("{}", dfs(start, target, 0, &graph_one, &mut visited));
    println!("{}", dfs(start, target, 0, &graph_two, &mut visited));
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
    while let Some((&pos, edges)) = graph.iter().find(|(_, es)| es.len() == 2)
    {
        let [(&p1, &c1), (&p2, &c2)] = edges.iter().next_chunk().unwrap();
        graph.remove(&pos);
        graph.entry(p1).and_modify(|edges| { edges.remove(&pos); edges.insert(p2, c1+c2); });
        graph.entry(p2).and_modify(|edges| { edges.remove(&pos); edges.insert(p1, c1+c2); });
    }
}

fn dfs(pos : Pos, target : Pos, steps : u32, graph : &Graph, visited : &mut HashSet<Pos>) -> u32
{
    if pos == target        { return steps }
    if !visited.insert(pos) { return 0     }

    let max = graph.get(&pos).unwrap()
                   .iter().map(|(&next, &cost)| dfs(next, target, steps+cost, graph, visited))
                   .max().unwrap_or(0);

    visited.remove(&pos);
    max
}
