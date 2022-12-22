use std::collections::HashMap;
use puzzle22::{ Pos, Tile, Inst, Facing };

fn main()
{
    let mut input = include_str!("../input.txt").split("\n\n");
    let grid      = Tile::parse_grid(input.next().unwrap_or(""));
    let path      = Inst::parse_path(input.next().unwrap_or(""));

    if let Some(mut pos) = grid.keys().min_by(|p, q| p.y.cmp(&q.y).then(p.x.cmp(&q.x))).copied()
    {
        let mut facing = Facing::Right;
        for inst in path
        {
            match inst
            {
                Inst::Move(steps) => for _ in 0 .. steps
                {
                    let next = pos + facing.offset();
                    match grid.get(&next)
                    {
                        Some(Tile::Open) => pos = next,
                        Some(Tile::Wall) => break,
                        None             =>
                        {
                            let wrap = flat_wrap(pos, facing, &grid);
                            if let Some(Tile::Open) = grid.get(&wrap) { pos = wrap } else { break }
                        }
                    }
                },
                Inst::Turn(turn)  => facing = facing.turn(turn)
            }
        }
        println!("{}", pos.x * 4 + pos.y * 1000 + facing as i32);
    }
}

fn flat_wrap(pos : Pos, facing : Facing, grid : &HashMap<Pos, Tile>) -> Pos
{
    pos.line(facing.opposite())
       .take_while(|prev| grid.contains_key(prev))
       .last().unwrap_or(pos)
}
