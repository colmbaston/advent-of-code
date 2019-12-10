mod intcode_core;

pub use intcode_core::*;
use std::thread;
use std::sync::mpsc::sync_channel;
use std::io::{ stdin, stdout, Write };

pub fn parse_file(fp : &str) -> Memory
{
    std::fs::read_to_string(fp).unwrap().trim_end().split(',').map(|x| x.parse().unwrap()).collect()
}

pub fn interpret_noio(memory : &mut Memory)
{
    let (send_d, recv_d) = sync_channel(0);
    interpret(memory, recv_d, send_d, None);
}

pub fn interpret_vecio(memory : &mut Memory, inputs : Vec<i64>) -> Vec<i64>
{
    let (send_in,  recv_in ) = sync_channel(1);
    let (send_out, recv_out) = sync_channel(1);

    let send_vec = thread::spawn(move ||
    {
        for i in inputs
        {
            if let Err(_) = send_in.send(i) { break }
        }
    });

    let recv_vec = thread::spawn(move ||
    {
        recv_out.iter().collect()
    });

    interpret(memory, recv_in, send_out, None);

    send_vec.join().unwrap();
    recv_vec.join().unwrap()
}

pub fn interpret_stdio(memory : &mut Memory)
{
    let (send_in,  recv_in ) = sync_channel(1);
    let (send_out, recv_out) = sync_channel(1);
    let (send_req, recv_req) = sync_channel(1);

    let stdio = thread::spawn(move ||
    {
        for r in recv_req.iter()
        {
            match r
            {
                Request::Output => if let Ok(x) = recv_out.recv() { println!("{}", x) },
                Request::Input  =>
                {
                    loop
                    {
                        print!("input: ");
                        stdout().flush().expect("failed to flush stdout");

                        let mut input = String::new();
                        stdin().read_line(&mut input).unwrap();
                        match input.trim_end().parse()
                        {
                            Ok(x)  => { send_in.send(x).unwrap(); break },
                            Err(_) => println!("parse error!")
                        }
                    }
                }
            }
        }
    });

    interpret(memory, recv_in, send_out, Some(send_req));
    stdio.join().unwrap();
}
