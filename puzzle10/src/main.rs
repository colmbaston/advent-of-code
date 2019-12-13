use num_integer::Integer;
use std::f64::consts::PI;
use std::collections::{ BTreeMap, BTreeSet };

fn main()
{
    let asteroids = coords(std::fs::read_to_string("input.txt").unwrap().chars(), |x| x == '#');

    if let Some(visible) = asteroids.iter().map(|a| detect(a, &asteroids)).max_by(|x, y| x.len().cmp(&y.len()))
    {
        println!("{}", visible.len());

        let mut angles = visible.iter().map(|(&v, c)| (vector_to_angle(v), c)).collect::<Vec<_>>();
        angles.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        println!("{}", { let x = angles[199].1; x.0 * 100 + x.1 });
    }
}

fn coords(iter : impl Iterator<Item = char>, f : impl Fn(char) -> bool) -> BTreeSet<(i64, i64)>
{
    let step = |((x, y), mut s) : (_, BTreeSet<_>), c|
    {
        match c
        {
            '\n' => ((0, y+1), s),
            _    => ((x+1, y), if f(c) { s.insert((x, y)) ; s } else { s })
        }
    };

    iter.fold(((0, 0), BTreeSet::new()), step).1
}

fn detect((x, y) : &(i64, i64), s : &BTreeSet<(i64, i64)>) -> BTreeMap<(i64, i64), (i64, i64)>
{
    s.iter().filter(|(a, b)| (a, b) != (x, y)).map(|&(a, b)|
    {
        let (dx, dy) = (x - a, y - b);
        let gcd      = dx.gcd(&dy);
        ((dx / gcd, dy / gcd), (a, b))
    })
    .fold(BTreeMap::new(), |mut m, ((a, b), (c, d))|
    {
        let (e, f) = m.entry((a, b)).or_insert((c, d));
        if (x-c).abs() < (x-*e).abs() || (y-d).abs() < (y-*f).abs()
        {
            *e = c;
            *f = d;
        }
        m
    })
}

fn vector_to_angle((a, b) : (i64, i64)) -> f64
{
    ((a as f64).atan2(-b as f64) + PI).rem_euclid(2.0 * PI)
}
