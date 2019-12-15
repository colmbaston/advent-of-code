use intcode;
use std::collections::{ HashSet, HashMap, VecDeque };
use std::sync::mpsc::{ sync_channel, SyncSender, Receiver };

fn main()
{
    let mut input = intcode::parse_file!("../input.txt");

    let (send_in,  recv_in)  = sync_channel(1);
    let (send_out, recv_out) = sync_channel(1);
    let drone  = std::thread::spawn(move || intcode::interpret(&mut input, recv_in, send_out, None));
    let canvas = explore_dfs(send_in, recv_out);
    drone.join().unwrap();

    if let (steps, Some(oxygen)) = bfs(&canvas, |pos| canvas.get(pos) == Some(&2), (0, 0))
    {
        println!("{}", steps);

        if let (steps, None) = bfs(&canvas, |_| false, oxygen)
        {
            println!("{}", steps);
        }
    }
}

fn ortho((x, y) : (i64, i64)) -> [(i64, i64) ; 4]
{
    [(x, y+1), (x, y-1), (x-1, y), (x+1, y)]
}

fn explore_dfs(send_in : SyncSender<i64>, recv_out : Receiver<i64>) -> HashMap<(i64, i64), i64>
{
    let mut stack  = Vec::new();
    let mut canvas = HashMap::new();
    let mut pos    = (0, 0);
    canvas.insert(pos, 1);

    'outer: loop
    {
        for (i, &dir) in ortho(pos).iter().enumerate()
        {
            if !canvas.contains_key(&dir)
            {
                send_in.send((i+1) as i64).unwrap();
                if let Ok(status) = recv_out.recv()
                {
                    canvas.insert(dir, status);
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
                send_in.send((dir - 1 ^ 1) + 1).unwrap();
                recv_out.recv().unwrap();
                pos
            }
        }
    }

    send_in.send(0).unwrap();
    canvas.retain(|_, x| *x != 0);
    canvas
}

fn bfs(canvas : &HashMap<(i64, i64), i64>, f : impl Fn(&(i64, i64)) -> bool, start : (i64, i64)) -> (u64, Option<(i64, i64)>)
{
    let mut visited = HashSet::new();
    let mut queue   = VecDeque::new();
    queue.push_back((0, start));

    loop
    {
        if let Some((steps, pos)) = queue.pop_front()
        {
            if f(&pos)
            {
                return (steps, Some(pos))
            }

            queue.extend(ortho(pos).iter().filter_map(|next|
            {
                canvas.get(next)?;
                if visited.contains(next) { None } else { Some((steps+1, *next)) }
            }));

            if queue.is_empty()
            {
                return (steps, None)
            }

            visited.insert(pos);
        }
    }
}
