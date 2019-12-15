use intcode::Request;
use std::cmp::Ordering;
use std::sync::mpsc::{ sync_channel, SyncSender, Receiver };

fn main()
{
    let mut code1 = intcode::parse_file!("../input.txt");
    let mut code2 = code1.clone();
    code2[0] = 2;

    let v = intcode::interpret_vecio(&mut code1, Vec::new());
    println!("{}", v.iter().skip(2).step_by(3).filter(|&x| *x == 2).count());

    let (send_in,  recv_in)  = sync_channel(1);
    let (send_out, recv_out) = sync_channel(1);
    let (send_req, recv_req) = sync_channel(1);
    let game = std::thread::spawn(move || run_game(send_in, recv_out, recv_req));
    intcode::interpret(&mut code2, recv_in, send_out, Some(send_req));
    game.join().unwrap();
}

fn run_game(send_in : SyncSender<i64>, recv_out : Receiver<i64>, recv_req : Receiver<Request>)
{
    let mut score  = 0;
    let mut ball_x = 0;
    let mut padd_x = 0;

    loop
    {
        match recv_req.recv()
        {
            Err(_)              => break,
            Ok(Request::Input)  =>
            {
                send_in.send(match ball_x.cmp(&padd_x)
                {
                    Ordering::Less    => -1,
                    Ordering::Equal   =>  0,
                    Ordering::Greater =>  1
                })
                .unwrap();
            },
            Ok(Request::Output) =>
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
    println!("{}", score);
}
