use std::collections::HashMap;
use aoc::direction::Direction;

fn main()
{
    let grid      = parse_grid(include_str!("../input.txt"));
    let mut pos   = grid.keys().min_by_key(|(x, y)| (y, x)).copied().unwrap();
    let mut dir   = Direction::South;
    let next_tile = move || match grid.get(&pos)
    {
        Some(Tile::Corner) =>
        {
            [dir.clockwise(), dir.anticlockwise()].into_iter().find_map(|next_dir|
            {
                let next_pos = next_dir.step(pos);
                grid.get(&next_pos).map(|_| { pos = next_pos; dir = next_dir; Tile::Corner })
            })
        },
        Some(&tile) =>
        {
            pos = dir.step(pos);
            Some(tile)
        },
        None => None
    };

    let mut count = 0;
    for tile in std::iter::from_fn(next_tile)
    {
        count += 1;
        if let Tile::Letter(b) = tile { print!("{}", b as char) }
    }
    println!();
    println!("{count}")
}

type Pos = (i32, i32);

#[derive(Copy, Clone)]
enum Tile
{
    Vertical,
    Horizontal,
    Corner,
    Letter(u8)
}

fn parse_grid(s : &str) -> HashMap<Pos, Tile>
{
    let mut grid = HashMap::new();
    for (l, y) in s.lines().zip(0 ..)
    {
        for (b, x) in l.bytes().zip(0 ..)
        {
            let pos = (x, y);
            match b
            {
                b'|' => { grid.insert(pos, Tile::Vertical);   },
                b'-' => { grid.insert(pos, Tile::Horizontal); },
                b'+' => { grid.insert(pos, Tile::Corner);     },
                _    => if b.is_ascii_alphabetic() { grid.insert(pos, Tile::Letter(b)); }
            }
        }
    }
    grid
}
