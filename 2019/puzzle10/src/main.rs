use num_integer::Integer;
use std::f64::consts::PI;
use std::collections::{ HashMap, HashSet };

fn main()
{
    let asteroids = coords(include_str!("../input.txt").bytes(), |x| x == b'#');

    if let Some(visible) = asteroids.iter().map(|a| detect(a, &asteroids)).max_by(|x, y| x.len().cmp(&y.len()))
    {
        println!("{}", visible.len());

        let mut angles = visible.iter().map(|(&v, c)| (vector_to_angle(v), c)).collect::<Vec<_>>();
        angles.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        println!("{}", { let x = angles[199].1; x.0 * 100 + x.1 });
    }
}

fn coords(iter : impl Iterator<Item = u8>, f : impl Fn(u8) -> bool) -> HashSet<(i64, i64)>
{
    let step = |((x, y), mut s) : (_, HashSet<_>), b|
    {
        match b
        {
            b'\n' => ((0, y+1), s),
            _     => ((x+1, y), if f(b) { s.insert((x, y)) ; s } else { s })
        }
    };

    iter.fold(((0, 0), HashSet::new()), step).1
}

fn detect((x, y) : &(i64, i64), s : &HashSet<(i64, i64)>) -> HashMap<(i64, i64), (i64, i64)>
{
    s.iter().filter(|(a, b)| (a, b) != (x, y)).map(|&(a, b)|
    {
        let (dx, dy) = (x - a, y - b);
        let gcd      = dx.gcd(&dy);
        ((dx / gcd, dy / gcd), (a, b))
    })
    .fold(HashMap::new(), |mut m, ((a, b), (c, d))|
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