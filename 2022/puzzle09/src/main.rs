use std::collections::HashSet;

mod pos;
use pos::{ Pos, Offset };

fn main()
{
    let offsets     = include_str!("../input.txt").lines().filter_map(Offset::parse).collect::<Vec<Offset>>();
    let mut rope    = Vec::new();
    let mut visited = HashSet::new();

    for knots in [2, 10]
    {
        rope.clear();
        rope.extend(std::iter::repeat_n(Pos::ORIGIN, knots));

        visited.clear();
        visited.insert(Pos::ORIGIN);

        for &Offset(axis, k) in &offsets
        {
            for _ in 0 .. k.abs()
            {
                if let Some(head) = rope.first_mut() { head[axis] += k.signum() }
                for ix in 1 .. knots
                {
                    let (lower, upper)                    = rope.split_at_mut(ix);
                    if let (Some(leader), Some(follower)) = (lower.last_mut(), upper.first_mut())
                    && let Some(pos) = follower.step_towards(*leader)
                    {
                        *follower = pos
                    }
                }
                if let Some(tail) = rope.last() { visited.insert(*tail); }
            }
        }
        println!("{}", visited.len());
    }
}
