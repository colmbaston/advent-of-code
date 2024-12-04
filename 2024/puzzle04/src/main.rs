fn main()
{
    let grid = include_str!("../input.txt").lines()
                                           .map(|l| l.bytes().collect())
                                           .collect::<Vec<Vec<u8>>>();

    let width  = grid[0].len();
    let height = grid.len();

    let mut count_one = 0;
    let mut count_two = 0;
    for y in 0 .. height
    {
        for x in 0 .. width
        {
            if y+3 < height
            {
                match (grid[y][x], grid[y+1][x], grid[y+2][x], grid[y+3][x])
                {
                    (b'X', b'M', b'A', b'S') => count_one += 1,
                    (b'S', b'A', b'M', b'X') => count_one += 1,
                    _                        => ()
                }
            }

            if x+3 < width
            {
                match (grid[y][x], grid[y][x+1], grid[y][x+2], grid[y][x+3])
                {
                    (b'X', b'M', b'A', b'S') => count_one += 1,
                    (b'S', b'A', b'M', b'X') => count_one += 1,
                    _                        => ()
                }
            }

            if y+3 < height && x+3 < width
            {
                match (grid[y][x], grid[y+1][x+1], grid[y+2][x+2], grid[y+3][x+3])
                {
                    (b'X', b'M', b'A', b'S') => count_one += 1,
                    (b'S', b'A', b'M', b'X') => count_one += 1,
                    _                        => ()
                }
            }

            if y+3 < height && x >= 3
            {
                match (grid[y][x], grid[y+1][x-1], grid[y+2][x-2], grid[y+3][x-3])
                {
                    (b'X', b'M', b'A', b'S') => count_one += 1,
                    (b'S', b'A', b'M', b'X') => count_one += 1,
                    _                        => ()

                }
            }

            if y+2 < height && x+2 < width
            {
                match (grid[y][x], grid[y+1][x+1], grid[y+2][x+2], grid[y+2][x], grid[y][x+2])
                {
                    (b'M', b'A', b'S', b'M', b'S') => count_two += 1,
                    (b'S', b'A', b'M', b'M', b'S') => count_two += 1,
                    (b'M', b'A', b'S', b'S', b'M') => count_two += 1,
                    (b'S', b'A', b'M', b'S', b'M') => count_two += 1,
                    _                              => ()
                }
            }
        }
    }
    println!("{count_one}");
    println!("{count_two}");
}
