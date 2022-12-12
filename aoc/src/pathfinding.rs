use std::{ ops::Add, cmp::Reverse, hash::Hash, collections::{ BinaryHeap, HashSet, VecDeque }};
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
    queue.extend(inits.map(|init| (Reverse((heuristic(&init), num_traits::identities::zero())), init)));

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
    a_star(inits, target, adjacent, |_| num_traits::identities::zero())
}

pub fn bfs<P, C, A>(inits    : impl Iterator<Item = P>,
                    target   : impl Fn(&P) -> bool,
                    adjacent : impl Fn(&P) -> A) -> Option<C>
                      where
                        P : Copy + Ord + Hash,
                        C : Copy + Ord + Zero + One + Add,
                        A : Iterator<Item = P>
{
    let mut queue   = VecDeque::new();
    let mut visited = HashSet::new();
    queue.extend(inits.map(|init| (num_traits::identities::zero(), init)));

    while let Some((c, p)) = queue.pop_front()
    {
        if !visited.insert(p) { continue       }
        if target(&p)         { return Some(c) }
        queue.extend(adjacent(&p).map(|p| (c + num_traits::identities::one(), p)));
    }
    None
}
