use std::collections::HashMap;
use puzzle22::{ Pos, Tile, Inst, Facing };

fn main()
{
    let mut input = include_str!("../input.txt").split("\n\n");
    let grid      = Tile::parse_grid(input.next().unwrap_or(""));
    let path      = Inst::parse_path(input.next().unwrap_or(""));

    if let Some(mut flat_pos) = grid.keys().min_by(|p, q| p.y.cmp(&q.y).then(p.x.cmp(&q.x))).copied()
    {
        let mut cube_pos    = flat_pos;
        let mut flat_facing = Facing::Right;
        let mut cube_facing = Facing::Right;
        for inst in path
        {
            match inst
            {
                Inst::Move(steps) =>
                {
                    walk(steps, &mut flat_pos, &mut flat_facing, &grid, flat_wrap);
                    walk(steps, &mut cube_pos, &mut cube_facing, &grid, cube_wrap);
                },
                Inst::Turn(turn) =>
                {
                    flat_facing = flat_facing.turn(turn);
                    cube_facing = cube_facing.turn(turn);
                }
            }
        }

        println!("{}", flat_pos.x * 4 + flat_pos.y * 1000 + flat_facing as i32);
        println!("{}", cube_pos.x * 4 + cube_pos.y * 1000 + cube_facing as i32);
    }
}

fn walk(steps  : u32,
        pos    : &mut Pos,
        facing : &mut Facing,
        grid   : &HashMap<Pos, Tile>,
        wrap   : impl Fn(Pos, Facing, &HashMap<Pos, Tile>) -> (Pos, Facing))
{
    for _ in 0 .. steps
    {
        let next = *pos + facing.offset();
        match grid.get(&next)
        {
            Some(Tile::Open) => *pos = next,
            Some(Tile::Wall) => break,
            None             =>
            {
                let (wrapped_pos, wrapped_facing) = wrap(*pos, *facing, grid);
                if let Some(Tile::Open) = grid.get(&wrapped_pos)
                {
                    *pos    = wrapped_pos;
                    *facing = wrapped_facing;
                }
                else
                {
                    break
                }
            }
        }
    }
}

fn flat_wrap(pos : Pos, facing : Facing, grid : &HashMap<Pos, Tile>) -> (Pos, Facing)
{
    (pos.line(facing.opposite())
        .take_while(|prev| grid.contains_key(prev))
        .last()
        .unwrap_or(pos), facing)
}

fn cube_wrap(pos : Pos, facing : Facing, _ : &HashMap<Pos, Tile>) -> (Pos, Facing)
{
    match (pos, facing)
    {
        (Pos { x: 150, y: y@(  1 ..=  50) }, Facing::Right) => (Pos { x:     100, y: 151 -   y }, Facing::Left),
        (Pos { x: 100, y: y@( 51 ..= 100) }, Facing::Right) => (Pos { x: y +  50, y:  50       }, Facing::Up),
        (Pos { x: 100, y: y@(101 ..= 150) }, Facing::Right) => (Pos { x:     150, y: 151 -   y }, Facing::Left),
        (Pos { x:  50, y: y@(151 ..= 200) }, Facing::Right) => (Pos { x: y - 100, y: 150       }, Facing::Up),

        (Pos { x:  51, y: y@(  1 ..=  50) }, Facing::Left)  => (Pos { x:       1, y: 151 -   y }, Facing::Right),
        (Pos { x:  51, y: y@( 51 ..= 100) }, Facing::Left)  => (Pos { x: y -  50, y: 101       }, Facing::Down),
        (Pos { x:   1, y: y@(101 ..= 150) }, Facing::Left)  => (Pos { x:      51, y: 151 -   y }, Facing::Right),
        (Pos { x:   1, y: y@(151 ..= 200) }, Facing::Left)  => (Pos { x: y - 100, y:   1       }, Facing::Down),

        (Pos { x: x@(  1 ..=  50), y: 101 }, Facing::Up)    => (Pos { x:      51, y:   x +  50 }, Facing::Right),
        (Pos { x: x@( 51 ..= 100), y:   1 }, Facing::Up)    => (Pos { x:       1, y:   x + 100 }, Facing::Right),
        (Pos { x: x@(101 ..= 150), y:   1 }, Facing::Up)    => (Pos { x: x - 100, y:       200 }, Facing::Up),

        (Pos { x: x@(  1 ..=  50), y: 200 }, Facing::Down)  => (Pos { x: x + 100, y:         1 }, Facing::Down),
        (Pos { x: x@( 51 ..= 100), y: 150 }, Facing::Down)  => (Pos { x:      50, y:   x + 100 }, Facing::Left),
        (Pos { x: x@(101 ..= 150), y:  50 }, Facing::Down)  => (Pos { x:     100, y:   x -  50 }, Facing::Left),

        _ => unreachable!()
    }
}
