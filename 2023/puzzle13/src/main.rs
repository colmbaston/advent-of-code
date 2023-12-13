fn main()
{
    let grids = include_str!("../input.txt").split("\n\n").map(parse_grid).collect::<Vec<Grid>>();
    println!("{}", grids.iter().filter_map(|g| summary(0, g)).sum::<usize>());
    println!("{}", grids.iter().filter_map(|g| summary(1, g)).sum::<usize>());
}

type Grid = Vec<Vec<bool>>;

fn parse_grid(s : &str) -> Grid
{
    s.lines()
     .map(|l| l.bytes()
               .map(|b| b == b'#')
               .collect())
     .collect()
}

fn summary(smudges : usize, grid : &Grid) -> Option<usize>
{
    reflect_h(smudges, grid).map(|rows| 100 * rows)
                            .or_else(|| reflect_v(smudges, grid))
}

fn reflect_h(smudges : usize, grid : &Grid) -> Option<usize>
{
    (1 .. grid.len()).find(|&row| smudges == grid[.. row].iter().rev()
                                                         .zip(grid[row ..].iter())
                                                         .map(|(a, b)| a.iter().zip(b.iter())
                                                                        .filter(|(c, d)| c != d)
                                                                        .count())
                                                         .sum())
}

fn reflect_v(smudges : usize, grid : &Grid) -> Option<usize>
{
    reflect_h(smudges, &aoc::transpose::transpose(grid.iter().map(Vec::as_slice)).collect::<Grid>())
}
