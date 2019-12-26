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
