mod direction;

use intcode::Interpreter;
use std::sync::mpsc::channel;
use std::collections::HashMap;

fn main()
{
    let input = intcode::parse_file!("../input.txt");

    for init in &[false, true]
    {
        let (send_in,  recv_in ) = channel();
        let (send_out, recv_out) = channel();
        let handle = Interpreter::with_channel(input.clone(), recv_in, send_out, None);

        let mut canvas    = HashMap::new();
        let mut direction = direction::Dir::Up;
        let mut position  = (0, 0);

        let _ = send_in.send(if *init { 1 } else { 0 });
        let mut out = recv_out.iter();
        while let (Some(c), Some(d)) = (out.next(), out.next())
        {
            canvas.insert(position, c == 1);
            if d == 1 { direction.turn_right() } else { direction.turn_left() }
            direction.advance(&mut position);
            let _ = send_in.send(if *canvas.entry(position).or_insert(false) { 1 } else { 0 });
        }
        handle.join().unwrap();

        if *init
        {
            canvas.retain(|_, v| *v);
            if let Some((min_x, min_y, max_x, max_y)) = aoc::bounds::bounds_2d(canvas.keys())
            {
                println!();
                for y in min_y ..= max_y
                {
                    print!(" ");
                    for x in min_x ..= max_x
                    {
                        print!("{}", if canvas.contains_key(&(x, y)) { '#' } else { ' ' });
                    }
                    println!();
                }
                println!();
            }
        }
        else
        {
            println!("{}", canvas.len());
        }
    }
}
