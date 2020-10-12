use std::collections::{ HashSet, VecDeque };

fn main()
{
    let facility = parse(include_str!("../input.txt").trim_end());

    // use a breadth-first search to find the shortest path to every room
    let mut visited = HashSet::new();
    let mut queue   = VecDeque::new();
    queue.push_back(((0, 0), 0));

    // keep track the farthest room and the number of rooms >= 1000 steps away
    let mut maximum = 0;
    let mut count   = 0;
    while let Some(((cx, cy), steps)) = queue.pop_front()
    {
        if !visited.insert((cx, cy)) { continue }

        if steps >= 1000    { count  += 1      }
        if steps >  maximum { maximum = steps }

        let offsets = vec![(0, -1), (1, 0), (0, 1), (-1, 0)];
        queue.extend(offsets.into_iter().filter_map(|(ox, oy)|
        {
            if facility.contains(&(cx+ox, cy+oy))
            {
                Some(((cx+ox+ox, cy+oy+oy), steps+1))
            }
            else
            {
                None
            }
        }));

    }

    println!("{}", maximum);
    println!("{}", count);
}

fn parse(s : &str) -> HashSet<(i32, i32)>
{
    let mut facility = HashSet::new();
    let mut stack    = Vec::new();
    let mut current  = (0, 0);

    for b in s.bytes()
    {
        match b
        {
            b'^' | b'$' => (),
            b'('        => stack.push(current),
            b')'        => current =  stack.pop().unwrap(),
            b'|'        => current = *stack.last().unwrap(),
            _           =>
            {
                let (ox, oy) = match b
                {
                    b'N' => ( 0, -1),
                    b'E' => ( 1,  0),
                    b'S' => ( 0,  1),
                    b'W' => (-1,  0),
                    _    => unreachable!()
                };

                let (cx, cy) = current;
                current      = (cx+ox+ox, cy+oy+oy);

                facility.insert(current);
                facility.insert((cx+ox, cy+oy));
            }
        }
    }

    facility
}
