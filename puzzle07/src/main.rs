use intcode::Memory;
use itertools::Itertools;
use std::{ thread, sync::mpsc::sync_channel };

fn main()
{
    let input = intcode::parse_file("input.txt");
    run(&input, false);
    run(&input, true);
}

fn run(input : &Memory, feedback : bool)
{
    let mut max = i64::min_value();
    for ps in if feedback { 5 .. 10 } else { 0 .. 5 }.permutations(5)
    {
        let (first_send, first_recv) = sync_channel(1);
        let mut last_send = first_send.clone();
        let mut last_recv = first_recv;

        for x in &ps
        {
            last_send.send(*x).unwrap();
            let (sender, receiver) = sync_channel(1);
            last_send = sender.clone();

            let mut memory = input.clone();
            thread::spawn(move || intcode::interpret(&mut memory, last_recv, sender, None));

            last_recv = receiver;
        }
        drop(last_send);

        first_send.send(0).unwrap();

        for x in last_recv.iter()
        {
            if max < x { max = x }
            if feedback { let _ = first_send.send(x); }
        }
    }
    println!("{}", max);
}
