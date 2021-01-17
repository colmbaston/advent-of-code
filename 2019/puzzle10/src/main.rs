use std::f64::consts::PI;
use std::collections::{ HashMap, HashSet };

fn main()
{
    let asteroids = coords(include_str!("../input.txt"));

    if let Some(visible) = asteroids.iter().map(|a| detect(a, &asteroids)).max_by(|x, y| x.len().cmp(&y.len()))
    {
        println!("{}", visible.len());

        let mut angles = visible.iter().map(|(&v, c)| (vector_to_angle(v), c)).collect::<Vec<_>>();
        angles.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        println!("{}", { let x = angles[199].1; x.0 * 100 + x.1 });
    }
}

fn coords(s : &str) -> HashSet<(i64, i64)>
{
    s.lines().zip(0..).flat_map(|(l, y)|
    {
        l.chars().zip(0..).filter_map(move |(c, x)|
        {
            if c == '#' { Some((x, y)) } else { None }
        })
    })
    .collect()
}

fn detect((x, y) : &(i64, i64), s : &HashSet<(i64, i64)>) -> HashMap<(i64, i64), (i64, i64)>
{
    s.iter().filter(|(a, b)| (a, b) != (x, y)).map(|&(a, b)|
    {
        let (dx, dy) = (x - a, y - b);

        let gcd =
        {
            let mut a = dx.abs();
            let mut b = dy.abs();

            while b > 0
            {
                let temp = a;
                a = b;
                b = temp % b;
            }

            a
        };

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
