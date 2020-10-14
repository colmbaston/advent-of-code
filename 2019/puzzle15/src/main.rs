use intcode::Interpreter;
use std::sync::mpsc::{ channel, Sender, Receiver };
use std::collections::{ VecDeque, HashSet, HashMap, hash_map::Entry };

fn main()
{
    let input = intcode::parse_file!("../input.txt");

    let (send_in,  recv_in)  = channel();
    let (send_out, recv_out) = channel();
    let handle = Interpreter::with_channel(input, recv_in, send_out, None);
    let canvas = explore_dfs(send_in, recv_out);
    handle.join().unwrap();

    // part 1: bfs from the origin to the oxygen
    let mut visited = HashSet::new();
    let mut queue   = VecDeque::new();
    queue.push_back(((0, 0), 0));
    let oxygen = loop
    {
        if let Some((pos, steps)) = queue.pop_front()
        {
            if !visited.insert(pos) { continue }
            if let Some(&2) = canvas.get(&pos)
            {
                println!("{}", steps);
                break pos
            }
            queue.extend(ortho(pos).filter_map(|p| canvas.get(&p).map(|_| (p, steps+1))));
        }
    };

    // part 2: bfs from the oxygen until the queue is exhausted
    let mut max = 0;
    visited.clear();
    queue.clear();
    queue.push_back((oxygen, 0));
    while let Some((pos, steps)) = queue.pop_front()
    {
        if !visited.insert(pos) { continue }
        if steps > max { max = steps}
        queue.extend(ortho(pos).filter_map(|p| canvas.get(&p).map(|_| (p, steps+1))));
    }
    println!("{}", max);
}

fn ortho((x, y) : (i64, i64)) -> impl Iterator<Item = (i64, i64)>
{
    vec![(x, y-1), (x, y+1), (x+1, y), (x-1, y)].into_iter()
}

fn explore_dfs(send_in : Sender<i64>, recv_out : Receiver<i64>) -> HashMap<(i64, i64), i64>
{
    let mut stack  = Vec::new();
    let mut canvas = HashMap::new();
    let mut pos    = (0, 0);
    canvas.insert(pos, 1);

    'outer: loop
    {
        for (dir, i) in ortho(pos).zip(1..)
        {
            if let Entry::Vacant(e) = canvas.entry(dir)
            {
                send_in.send(i).unwrap();
                if let Ok(status) = recv_out.recv()
                {
                    e.insert(status);
                    if status != 0
                    {
                        stack.push((pos, i));
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
