use intcode::Interpreter;
use std::sync::mpsc::{ channel, Sender, Receiver };
use std::collections::{ HashMap, hash_map::Entry };

fn main()
{
    let input = intcode::parse_file!("../input.txt");

    let (send_in,  recv_in)  = channel();
    let (send_out, recv_out) = channel();
    let handle = Interpreter::with_channel(input, recv_in, send_out, None);
    let canvas = explore_dfs(send_in, recv_out);
    handle.join().unwrap();

    let adjacent = |c : &(i64, i64)| ortho(*c).into_iter().filter(|c| canvas.get(&c).is_some());
    if let (steps, Some((oxygen, _))) = search::bfs((0, 0), adjacent, |c| canvas.get(c) == Some(&2), |_| None::<()>)
    {
        println!("{}", steps);

        if let (steps, None) = search::bfs(oxygen, adjacent, |_| false, |_| None::<()>)
        {
            println!("{}", steps);
        }
    }
}

fn ortho((x, y) : (i64, i64)) -> Vec<(i64, i64)>
{
    vec![(x, y+1), (x, y-1), (x-1, y), (x+1, y)]
}

fn explore_dfs(send_in : Sender<i64>, recv_out : Receiver<i64>) -> HashMap<(i64, i64), i64>
{
    let mut stack  = Vec::new();
    let mut canvas = HashMap::new();
    let mut pos    = (0, 0);
    canvas.insert(pos, 1);

    'outer: loop
    {
        for (i, dir) in ortho(pos).into_iter().enumerate()
        {
            if let Entry::Vacant(e) = canvas.entry(dir)
            {
                send_in.send((i+1) as i64).unwrap();
                if let Ok(status) = recv_out.recv()
                {
                    e.insert(status);
                    if status != 0
                    {
                        stack.push((pos, (i+1) as i64));
                        pos = dir;
                        continue 'outer
                    }
                }
            }
        }

        pos = match stack.pop()
        {
            None             => break,
            Some((pos, dir)) =>
            {
                send_in.send(((dir - 1) ^ 1) + 1).unwrap();
                recv_out.recv().unwrap();
                pos
            }
        }
    }

    send_in.send(0).unwrap();
    canvas.retain(|_, x| *x != 0);
    canvas
}
