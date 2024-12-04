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
                count_one += matches!(&[grid[y][x], grid[y+1][x], grid[y+2][x], grid[y+3][x]], b"XMAS" | b"SAMX") as u32;
            }
            if x+3 < width
            {
                count_one += matches!(&[grid[y][x], grid[y][x+1], grid[y][x+2], grid[y][x+3]], b"XMAS" | b"SAMX") as u32;
            }
            if y+3 < height && x+3 < width
            {
                count_one += matches!(&[grid[y][x], grid[y+1][x+1], grid[y+2][x+2], grid[y+3][x+3]], b"XMAS" | b"SAMX") as u32;
            }
            if y+3 < height && x >= 3
            {
                count_one += matches!(&[grid[y][x], grid[y+1][x-1], grid[y+2][x-2], grid[y+3][x-3]], b"XMAS" | b"SAMX") as u32;
            }
            if y+2 < height && x+2 < width
            {
                count_two += matches!(&[grid[y][x], grid[y+1][x+1], grid[y+2][x+2], grid[y+2][x], grid[y][x+2]], b"MASMS" | b"SAMMS" | b"MASSM" | b"SAMSM") as u32;
            }
        }
    }
    println!("{count_one}");
    println!("{count_two}");
}
