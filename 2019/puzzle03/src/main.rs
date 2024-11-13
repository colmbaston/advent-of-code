mod coords;
use coords::Coords;

fn main()
{
    let input  = include_str!("../input.txt");
    let mut ls = input.lines();
    let mut parse_next = || ls.next().unwrap().split(',').scan(((0,0), 0), |a, b| Some(parse_segment(a, b))).collect::<Vec<Segment>>();
    let wire_one = parse_next();
    let wire_two = parse_next();

    let mut minhattan = u64::MAX;
    let mut min_steps = u64::MAX;

    for i in &wire_one
    {
        for j in &wire_two
        {
            for (x, y) in intersection(i, j)
            {
                if (x, y) == (0, 0) { continue }

                let vi = if i.vertical { y } else { x };
                let vj = if j.vertical { y } else { x };

                let steps = i.length + j.length + (vi - i.start).unsigned_abs() + (vj - j.start).unsigned_abs();
                minhattan = minhattan.min((x.abs() + y.abs()) as u64);
                min_steps = min_steps.min(steps);
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

fn parse_segment(((sx, sy), a) : &mut ((i64, i64), u64), s : &str) -> Segment
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

    *a  += l as u64;
    if v { *sy = e } else { *sx = e };

    result
}

fn intersection(p : &Segment, q : &Segment) -> Coords
{
    let p_min = p.start.min(p.end);
    let q_min = q.start.min(q.end);
    let p_max = p.start.max(p.end);
    let q_max = q.start.max(q.end);

    let pq_min = p_min.max(q_min);
    let pq_max = p_max.min(q_max);

    if p.vertical == q.vertical
    {
        return if p.fixed == q.fixed && pq_min <= pq_max
        {
            if p.vertical
            {
                Coords::FixedX(p.fixed, pq_min, pq_max)
            }
            else
            {
                Coords::FixedY(p.fixed, pq_min, pq_max)
            }
        }
        else
        {
            Coords::Empty
        }
    }

    if q_min <= p.fixed && p.fixed <= q_max && p_min <= q.fixed && q.fixed <= p_max
    {
        let (v, h) = if p.vertical { (p, q) } else { (q, p) };
        return Coords::Single(v.fixed, h.fixed)
    }

    Coords::Empty
}
