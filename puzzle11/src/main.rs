mod direction;

use intcode;
use itertools::{ Itertools, MinMaxResult };
use std::collections::BTreeMap;
use std::sync::mpsc::{ sync_channel, SyncSender, Receiver };

fn main()
{
    let input = intcode::parse_file!("../input.txt");

    for init in &[false, true]
    {
        let mut code             = input.clone();
        let (send_in,  recv_in ) = sync_channel(1);
        let (send_out, recv_out) = sync_channel(1);

        let robot = std::thread::spawn(move || run_robot(*init, send_in, recv_out));
        intcode::interpret(&mut code, recv_in, send_out, None);

        if let Ok(canvas) = robot.join()
        {
            if *init
            {
                if let (MinMaxResult::MinMax(&(min_x, _), &(max_x, _)), MinMaxResult::MinMax(&(_, min_y), &(_, max_y))) = (canvas.keys().minmax_by(|a, b| a.0.cmp(&b.0)), canvas.keys().minmax_by(|a, b| a.1.cmp(&b.1)))
                {
                    print!("\n");
                    for y in min_y .. max_y + 1
                    {
                        print!(" ");
                        for x in min_x .. max_x + 1
                        {
                            print!("{}", match canvas.get(&(x, y)) { None => ' ', Some(&c) => if c { '#' } else { ' ' }});
                        }
                        print!("\n");
                    }
                    print!("\n");
                }
            }
            else
            {
                println!("{}", canvas.len());
            }
        }
    }
}

fn run_robot(init : bool, send_in : SyncSender<i64>, recv_out : Receiver<i64>) -> BTreeMap<(i64, i64), bool>
{
    let mut canvas    = BTreeMap::new();
    let mut direction = direction::Dir::Up;
    let mut position  = (0, 0);

    let _ = send_in.send(if init { 1 } else { 0 });
    for (c, d) in recv_out.iter().tuples()
    {
        canvas.insert(position, c == 1);
        if d == 1 { direction.turn_right() } else { direction.turn_left() }
        direction.advance(&mut position);
        let _ = send_in.send(if *canvas.entry(position).or_insert(false) { 1 } else { 0 });
    }

    canvas
}
