use std::collections::{ HashSet, VecDeque };

struct Ortho
{
    origin  : (i64, i64),
    current : u8
}

impl Iterator for Ortho
{
    type Item = (i64, i64);

    fn next(&mut self) -> Option<(i64, i64)>
    {
        let (x, y) = self.origin;
        let result = match self.current % 4
        {
            0 => Some((x+1, y  )),
            1 => Some((x-1, y  )),
            2 => Some((x,   y+1)),
            3 => Some((x,   y-1)),
            _ => panic!("impossible")
        };
        self.current += 1;

        result
    }
}

pub fn ortho(origin : (i64, i64)) -> impl Iterator<Item = (i64, i64)>
{
    Ortho { origin, current: 0 }.take(4)
}

pub fn bfs<S, I, P>(start : S, adjacent : impl Fn(&S) -> I, complete : impl Fn(&S) -> bool, append_path : impl Fn(&S) -> Option<P>) -> (u64, Option<(S, Vec<P>)>)
where S : Eq + std::hash::Hash, I : Iterator<Item = S>, P : Clone
{
    let mut visited = HashSet::new();
    let mut queue   = VecDeque::new();
    queue.push_back((0, start, Vec::new()));

    loop
    {
        if let Some((steps, state, path)) = queue.pop_front()
        {
            if complete(&state)
            {
                return (steps, Some((state, path)))
            }

            queue.extend(adjacent(&state).filter_map(|state|
            {
                if visited.contains(&state)
                {
                    None
                }
                else
                {
                    let mut path = path.clone();
                    if let Some(x) = append_path(&state) { path.push(x) }
                    Some((steps+1, state, path))
                }
            }));

            if queue.is_empty()
            {
                return (steps, None)
            }

            visited.insert(state);
        }
    }
}
