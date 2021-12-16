use std::hash::Hash;
use std::cmp::Reverse;
use std::collections::{ BinaryHeap, HashSet };

pub fn ortho_2d<T : num::Num + Copy>(x : T, y : T) -> Vec<(T, T)>
{
    let one = num::one();

    vec![(x - one, y), (x + one, y),
         (x, y - one), (x, y + one)]
}

pub fn ortho_3d<T : num::Num + Copy>(x : T, y : T, z : T) -> Vec<(T, T, T)>
{
    let one = num::one();

    vec![(x - one, y, z), (x + one, y, z),
         (x, y - one, z), (x, y + one, z),
         (x, y, z - one), (x, y, z + one)]
}

pub fn a_star<P, C>(init : P, target : impl Fn(&P) -> bool, adj : impl Fn(&P) -> Vec<(P, C)>, h : impl Fn(&P) -> C) -> Option<C>
where P : Copy + Ord + Hash, C : Copy + Ord + num::Num
{
    let mut queue   = BinaryHeap::new();
    let mut visited = HashSet::new();

    queue.push((Reverse((h(&init), num::zero())), init));
    while let Some((Reverse((_, c)), p)) = queue.pop()
    {
        if !visited.insert(p) { continue       }
        if target(&p)         { return Some(c) }

        queue.extend(adj(&p).into_iter().map(|(p, d)| (Reverse((c+d+h(&p), c+d)), p)));
    }
    None
}

pub fn dijkstra<P, C>(init : P, target : impl Fn(&P) -> bool, adjacent : impl Fn(&P) -> Vec<(P, C)>) -> Option<C>
where P : Copy + Ord + Hash, C : Copy + Ord + num::Num
{
    a_star(init, target, adjacent, |_| num::zero())
}
