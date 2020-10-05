const SIZE : usize = 300;
type Grid = [[i64 ; SIZE+1] ; SIZE+1];

fn main()
{
    let grid = initialise_grid(include_str!("../input.txt").trim_end().parse().unwrap());

    let (_, (x, y)) = power_square(&grid, 3);
    println!("{},{}", x, y);

    let (s, (_, (x, y))) = (1 ..= SIZE).map(|s| (s, power_square(&grid, s))).max_by_key(|(_, (k, _))| *k).unwrap();
    println!("{},{},{}", x, y, s);
}

fn initialise_grid(serial : i64) -> Grid
{
    let mut grid = [[0 ; SIZE+1] ; SIZE+1];

    for x in 1 ..= SIZE
    {
        let rack_id = (x + 10) as i64;
        for y in 1 ..= SIZE
        {
            grid[x][y] = (rack_id * y as i64 + serial) * rack_id / 100 % 10 - 5
                       + grid[x-1][y]
                       + grid[x][y-1]
                       - grid[x-1][y-1];
        }
    }

    grid
}

fn power_square(grid : &Grid, square_size : usize) -> (i64, (usize, usize))
{
    let mut max  = i64::MIN;
    let mut best = (0, 0);

    for x in square_size ..= SIZE
    {
        for y in square_size ..= SIZE
        {
            let total = grid[x][y]
                      - grid[x-square_size][y]
                      - grid[x][y-square_size]
                      + grid[x-square_size][y-square_size];

            if total > max
            {
                max  = total;
                best = (x-square_size+1, y-square_size+1);
            }
        }
    }

    (max, best)
}
