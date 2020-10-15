use intcode::Interpreter;
use std::sync::mpsc::channel;

mod permutations;
use permutations::Permutations;

fn main()
{
    let input = intcode::parse_file!("../input.txt");
    run(&input, false);
    run(&input, true);
}

fn run(input : &[i64], feedback : bool)
{
    let mut max = std::i64::MIN;
    for perm in Permutations::new(if feedback { 5 .. 10 } else { 0 .. 5 })
    {
        let (first_send, first_recv) = channel();
        let mut last_recv = first_recv;
        let mut last_send = first_send.clone();
        let mut handles   = Vec::new();

        for x in perm.into_iter()
        {
            last_send.send(x).unwrap();
            let (sender, receiver) = channel();
            last_send = sender.clone();
            handles.push(Interpreter::with_channel(input.to_vec(), last_recv, sender, None));
            last_recv = receiver;
        }

        drop(last_send);
        first_send.send(0).unwrap();

        for x in last_recv.iter()
        {
            if max < x { max = x }
            if feedback { let _ = first_send.send(x); }
        }

        while let Some(h) = handles.pop()
        {
            h.join().unwrap();
        }
    }
    println!("{}", max);
}
