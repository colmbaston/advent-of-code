use std::{ ops::Add, cmp::Reverse, hash::Hash, collections::{ BinaryHeap, HashSet, HashMap, VecDeque }};
use num_traits::{ Zero, One };

pub fn a_star<P, C, A>(inits     : impl Iterator<Item = P>,
                       target    : impl Fn(&P) -> bool,
                       adjacent  : impl Fn(&P) -> A,
                       heuristic : impl Fn(&P) -> C) -> Option<C>
                         where
                           P : Copy + Ord + Hash,
                           C : Copy + Ord + Zero + Add,
                           A : Iterator<Item = (P, C)>
{
    let mut queue   = BinaryHeap::new();
    let mut visited = HashSet::new();
    queue.extend(inits.map(|init| (Reverse((heuristic(&init), C::zero())), init)));

    while let Some((Reverse((_, c)), p)) = queue.pop()
    {
        if !visited.insert(p) { continue       }
        if target(&p)         { return Some(c) }
        queue.extend(adjacent(&p).map(|(p, d)| (Reverse((c+d+heuristic(&p), c+d)), p)));
    }
    None
}

pub fn dijkstra<P, C, A>(inits    : impl Iterator<Item = P>,
                         target   : impl Fn(&P) -> bool,
                         adjacent : impl Fn(&P) -> A) -> Option<C>
                           where
                             P : Copy + Ord + Hash,
                             C : Copy + Ord + Zero + Add,
                             A : Iterator<Item = (P, C)>
{
    a_star(inits, target, adjacent, |_| C::zero())
}

pub fn bfs<P, C, A>(inits    : impl Iterator<Item = P>,
                    target   : impl Fn(&P) -> bool,
                    adjacent : impl Fn(&P) -> A) -> Option<C>
                      where
                        P : Copy + Eq + Hash,
                        C : Copy + Zero + One + Add,
                        A : Iterator<Item = P>
{
    let mut queue   = VecDeque::new();
    let mut visited = HashSet::new();
    queue.extend(inits.map(|init| (C::zero(), init)));

    while let Some((c, p)) = queue.pop_front()
    {
        if !visited.insert(p) { continue       }
        if target(&p)         { return Some(c) }
        queue.extend(adjacent(&p).map(|p| (c + C::one(), p)));
    }
    None
}

pub fn floyd_warshall<V, C>(vertices : impl Iterator<Item = V> + Clone, edges : impl Iterator<Item = ((V, V), C)>) -> HashMap<(V, V), C>
  where
    V : Copy + Eq + Hash,
    C : Copy + Ord + Add + Zero
{
    let mut dists = edges.collect::<HashMap<(V, V), C>>();

    for v in vertices.clone()
    {
        dists.insert((v, v), C::zero());
    }

    for t in vertices.clone()
    {
        for u in vertices.clone()
        {
            for v in vertices.clone()
            {
                if let Some(sum) = dists.get(&(t, u)).and_then(|&c| dists.get(&(t, v)).map(|&d| c + d))
                {
                    let current = dists.entry((u, v)).or_insert(sum);
                    *current = sum.min(*current);
                }
            }
        }
    }

    dists
}
