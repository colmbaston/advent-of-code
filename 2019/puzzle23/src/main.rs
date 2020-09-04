use std::time::Duration;
use intcode::Interpreter;
use std::sync::mpsc::channel;
use std::collections::HashSet;

const THREADS : usize = 50;

fn main()
{
    let input = intcode::parse_file!("../input.txt");

    let mut threads = Vec::with_capacity(THREADS);
    for i in 0 .. THREADS
    {
        let (send_in, recv_in) = channel();
        let interpreter        = Interpreter::new(input.clone(), std::iter::from_fn(move || recv_in.recv_timeout(Duration::new(0, 0)).ok()));

        send_in.send(i as i64).unwrap();
        threads.push((interpreter, send_in));
    }

    let mut nat              = None;
    let mut nat_last         = None;
    let mut nat_intervention = false;
    loop
    {
        let mut active : HashSet<usize> = (0 .. THREADS).collect();
        while let Some(i) = active.iter().next().copied()
        {
            let mut interpreter = threads[i as usize].0.iter();
            match interpreter.next()
            {
                None    => { active.remove(&i); },
                Some(a) =>
                {
                    let x = interpreter.next().unwrap();
                    let y = interpreter.next().unwrap();

                    if a == 255
                    {
                        if nat.is_none() { println!("{}", y) }
                        nat = Some((x, y));
                    }
                    else
                    {
                        let send_in = &threads[a as usize].1;
                        send_in.send(x).unwrap();
                        send_in.send(y).unwrap();

                        active.insert(a as usize);
                    }
                    nat_intervention = false;
                }
            }
        }

        match nat
        {
            Some((x, y)) if nat_intervention =>
            {
                if nat_last == Some(y) { println!("{}", y); break }

                let send_in = &threads[0].1;
                send_in.send(x).unwrap();
                send_in.send(y).unwrap();

                nat_last         = Some(y);
                nat_intervention = false;
            },
            _ =>
            {
                threads.iter().for_each(|(_, send_in)| send_in.send(-1).unwrap());
                nat_intervention = true;
            }
        }
    }
}
