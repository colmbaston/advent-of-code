use std::cmp::Ordering;
use std::sync::mpsc::channel;
use intcode::{ Interpreter, Request };

fn main()
{
    let mut input = intcode::parse_file!("../input.txt");

    println!("{}", Interpreter::new(input.clone(), std::iter::empty()).iter().skip(2).step_by(3).filter(|&x| x == 2).count());

    input[0] = 2;
    let (send_in,  recv_in)  = channel();
    let (send_out, recv_out) = channel();
    let (send_req, recv_req) = channel();
    let handle = Interpreter::with_channel(input, recv_in, send_out, Some(send_req));

    let mut score  = 0;
    let mut ball_x = 0;
    let mut padd_x = 0;

    for r in recv_req.iter()
    {
        match r
        {
            Request::Input =>
            {
                send_in.send(match ball_x.cmp(&padd_x)
                {
                    Ordering::Less    => -1,
                    Ordering::Equal   =>  0,
                    Ordering::Greater =>  1
                })
                .unwrap();
            },
            Request::Output =>
            {
                let x = recv_out.recv().unwrap();
                recv_req.recv().unwrap();
                let y = recv_out.recv().unwrap();
                recv_req.recv().unwrap();
                let t = recv_out.recv().unwrap();

                if x == -1 && y == 0
                {
                    score = t;
                }
                else if t == 3
                {
                    padd_x = x;
                }
                else if t == 4
                {
                    ball_x = x;
                }
            }
        }
    }
    handle.join().unwrap();

    println!("{}", score);
}
