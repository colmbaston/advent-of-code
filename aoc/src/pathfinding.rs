use std::{ ops::Add, cmp::Ordering, hash::Hash, collections::{ BinaryHeap, HashSet, HashMap, VecDeque }};
use num_traits::{ Zero, One };

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
        if target(&p)         { return Some(c) }
        if !visited.insert(p) { continue       }
        queue.extend(adjacent(&p).filter(|p| !visited.contains(p))
                                 .map(|p| (c + C::one(), p)));
    }
    None
}

pub fn dijkstra<P, C, A>(inits    : impl Iterator<Item = P>,
                         target   : impl Fn(&P) -> bool,
                         adjacent : impl Fn(&P) -> A) -> Option<C>
                           where
                             P : Copy + Eq  + Hash,
                             C : Copy + Ord + Zero + Add,
                             A : Iterator<Item = (P, C)>
{
    let mut queue   = BinaryHeap::new();
    let mut visited = HashSet::new();
    queue.extend(inits.map(|init| HeapNode { payload: init, cost: C::zero() }));

    while let Some(HeapNode { payload, cost }) = queue.pop()
    {
        if target(&payload)         { return Some(cost) }
        if !visited.insert(payload) { continue          }
        queue.extend(adjacent(&payload).filter(|(p, _)| !visited.contains(p))
                                       .map(|(p, c)| HeapNode { payload: p, cost: cost+c }))

    }
    None
}

pub fn a_star<P, C, A>(inits     : impl Iterator<Item = P>,
                       target    : impl Fn(&P) -> bool,
                       adjacent  : impl Fn(&P) -> A,
                       heuristic : impl Fn(&P) -> C + Copy) -> Option<C>
                         where
                           P : Copy + Eq  + Hash,
                           C : Copy + Ord + Zero + Add,
                           A : Iterator<Item = (P, C)>
{
    let mut queue   = BinaryHeap::new();
    let mut visited = HashSet::new();
    queue.extend(inits.map(|init| HeapNode { payload: init, cost: (heuristic(&init), C::zero()) }));

    while let Some(HeapNode { payload, cost: (_, cost) }) = queue.pop()
    {
        if target(&payload)         { return Some(cost) }
        if !visited.insert(payload) { continue          }
        queue.extend(adjacent(&payload).filter(|(p, _)| !visited.contains(p))
                                       .map(|(p, c)| HeapNode { payload: p, cost: (cost+c+heuristic(&p), cost+c) }))

    }
    None
}

struct HeapNode<P, C>
{
    payload: P,
    cost:    C
}

impl<P, C : Eq> Eq for HeapNode<P, C> {}

impl<P, C : Eq> PartialEq for HeapNode<P, C>
{
    fn eq(&self, other : &HeapNode<P, C>) -> bool
    {
        self.cost == other.cost
    }
}

impl<P, C : Ord> Ord for HeapNode<P, C>
{
    fn cmp(&self, other : &HeapNode<P, C>) -> Ordering
    {
        other.cost.cmp(&self.cost)
    }
}

impl<P, C : Ord> PartialOrd for HeapNode<P, C>
{
    fn partial_cmp(&self, other : &HeapNode<P, C>) -> Option<Ordering>
    {
        Some(self.cmp(other))
    }
}
