mod coords;
use crate::coords::{ Coords, Coords::* };
use std::cmp::{ min, max };

fn main()
{
    let input  = std::fs::read_to_string("input.txt").unwrap();
    let mut ls = input.lines();
    let mut parse_next = || ls.next().unwrap().split(',').scan(((0,0), 0), parse_segment).collect::<Vec<Segment>>();
    let wire_one = parse_next();
    let wire_two = parse_next();

    let mut minhattan = u64::max_value();
    let mut min_steps = u64::max_value();

    for i in &wire_one
    {
        for j in &wire_two
        {
            for (x, y) in intersection(i, j)
            {
                if (x, y) == (0, 0) { continue }

                let vi = if i.vertical { y } else { x };
                let vj = if j.vertical { y } else { x };

                let steps = i.length + j.length + (vi - i.start).abs() as u64 + (vj - j.start).abs() as u64;
                minhattan = min(minhattan, (x.abs() + y.abs()) as u64);
                min_steps = min(min_steps, steps);
            }
        }
    }

    println!("{}\n{}", minhattan, min_steps);
}

struct Segment
{
    length   : u64,
    vertical : bool,
    fixed    : i64,
    start    : i64,
    end      : i64
}

fn parse_segment(((sx, sy), a) : &mut ((i64, i64), u64), s : &str) -> Option<Segment>
{
    let (t, l)  = s.split_at(1);
    let l : i64 = l.parse().unwrap();

    let (v, f, s, e) = match t
    {
        "U" => (true,  *sx, *sy, *sy + l),
        "D" => (true,  *sx, *sy, *sy - l),
        "L" => (false, *sy, *sx, *sx - l),
        "R" => (false, *sy, *sx, *sx + l),
         _  => panic!("invalid direction!")
    };

    let result = Segment { length: *a, vertical: v, fixed: f, start: s, end: e };

    *a  = *a + l as u64;
    if v { *sy = e } else { *sx = e };

    Some(result)
}

fn intersection(p : &Segment, q : &Segment) -> Coords
{
    let p_min = min(p.start, p.end);
    let q_min = min(q.start, q.end);
    let p_max = max(p.start, p.end);
    let q_max = max(q.start, q.end);

    let pq_min = max(p_min, q_min);
    let pq_max = min(p_max, q_max);

    if p.vertical == q.vertical
    {
        return if p.fixed == q.fixed && pq_min <= pq_max
        {
            if p.vertical
            {
                FixedX(p.fixed, pq_min, pq_max)
            }
            else
            {
                FixedY(p.fixed, pq_min, pq_max)
            }
        }
        else
        {
            Empty
        }
    }

    if q_min <= p.fixed && p.fixed <= q_max && p_min <= q.fixed && q.fixed <= p_max
    {
        let (v, h) = if p.vertical { (p, q) } else { (q, p) };
        return Single(v.fixed, h.fixed)
    }

    Empty
}
