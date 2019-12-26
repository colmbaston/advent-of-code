use std::collections::{ HashSet, VecDeque };

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

struct Ortho
{
    origin  : (i64,  i64),
    current : (bool, bool)
}

impl Iterator for Ortho
{
    type Item = (i64, i64);

    fn next(&mut self) -> Option<(i64, i64)>
    {
        match self.current
        {
            (false, false) => { self.current = (false, true);  Some((self.origin.0 + 1, self.origin.1))     },
            (false, true)  => { self.current = (true,  false); Some((self.origin.0 - 1, self.origin.1))     },
            (true,  false) => { self.current = (true,  true);  Some((self.origin.0,     self.origin.1 + 1)) },
            (true,  true)  => { self.current = (false, false); Some((self.origin.0,     self.origin.1 - 1)) }
        }
    }
}

pub fn ortho(origin : (i64, i64)) -> impl Iterator<Item = (i64, i64)>
{
    Ortho { origin, current : (false, false) }.take(4)
}

pub fn ortho_origin(origin : (i64, i64)) -> impl Iterator<Item = ((i64, i64), (i64, i64))>
{
    Ortho { origin, current : (false, false) }.map(move |c| (origin, c)).take(4)
}
