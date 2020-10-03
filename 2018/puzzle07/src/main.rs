use std::cmp::Reverse;
use std::collections::{ BTreeMap, HashSet, BinaryHeap };

fn main()
{
    // build the dependency graph using a BTreeMap
    // since the ordering of the keys will matter
    let mut graph = BTreeMap::new();
    for c in b'A' ..= b'Z'
    {
        graph.insert(c, HashSet::new());
    }
    for s in include_str!("../input.txt").lines().map(|s| s.as_bytes())
    {
        graph.get_mut(&s[36]).unwrap().insert(s[5]);
    }

    let sorted  = topological_sort(graph.clone(), 1).0;
    let seconds = topological_sort(graph,         5).1;
    println!("{}", std::str::from_utf8(&sorted).unwrap());
    println!("{}", seconds);
}

fn topological_sort(mut graph : BTreeMap<u8, HashSet<u8>>, workers : usize) -> (Vec<u8>, u32)
{
    let mut seconds = 0;
    let mut sorted  = Vec::with_capacity(graph.len());
    let mut queue   = BinaryHeap::<(Reverse<u32>, _)>::with_capacity(workers);

    while !graph.is_empty()
    {
        match graph.iter().find(|(_, v)| v.is_empty())
        {
            Some((&k, _)) if queue.len() < workers =>
            {
                // there is a free worker to start step k
                sorted.push(k);
                queue.push((Reverse((k - b'A' + 61) as u32), k));
                graph.remove(&k);
            },
            _ =>
            {
                // the next step is blocked, so wait t seconds for one to finish
                let (Reverse(t), k) = queue.pop().unwrap();
                seconds += t;

                // remove the step that has just finished from the other steps' dependencies
                for (_, v) in graph.iter_mut() { v.remove(&k); }

                // reduce the time to wait for the other steps
                // BinaryHeap has no iter_mut(), so transmute it to its internal Vec representation
                // the function applied to each entry is monotonic, so the heap property will not be violated
                unsafe
                {
                    use std::mem::transmute;
                    let mut queue_vec : Vec<(Reverse<u32>, u8)> = transmute(queue);
                    for (Reverse(s), _) in queue_vec.iter_mut() { *s -= t }
                    queue = transmute(queue_vec);
                }
            }
        }
    }

    // all steps have now been queued, so wait for them to finish
    seconds += queue.iter().map(|(Reverse(t), _)| t).max().unwrap_or(&0);

    (sorted, seconds)
}
