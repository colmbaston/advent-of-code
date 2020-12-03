fn main()
{
    let input  = include_str!("../input.txt").lines().map(|l| l.chars().map(|c| c == '#').collect::<Vec<_>>()).collect::<Vec<_>>();

    println!("{}", trees(&input, 3, 1));
    println!("{}", [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)].iter().map(|&(n, d)| trees(&input, n, d)).product::<usize>());
}

fn trees(grid : &[Vec<bool>], right : usize, down : usize) -> usize
{
    let width  = grid[0].len();
    let height = grid.len();

    (0 ..).map(|k| (k * right, k * down))
          .take_while(|&(_, y)| y < height)
          .filter(|&(x, y)| grid[y][x % width])
          .count()
}
