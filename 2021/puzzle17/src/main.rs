fn main()
{
    let (lx, ux, ly, uy) = parse_target(include_str!("../input.txt"));

    let (count, my) = ((2*lx).isqrt() ..= ux).flat_map(|vx| (ly ..= -ly).map(move |vy| (vx, vy)))
                                             .filter_map(|(vx, vy)| simulate(vx, vy, lx, ux, ly, uy))
                                             .fold((0, 0), |(count, my), py| (count+1, my.max(py)));

    println!("{}", my);
    println!("{}", count);
}

fn parse_target(s : &str) -> (i32, i32, i32, i32)
{
    let mut i = s.split(|c : char| !(c.is_ascii_digit() || c == '-')).filter(|t| !t.is_empty());

    let lx = i.next().unwrap().parse().unwrap();
    let ux = i.next().unwrap().parse().unwrap();
    let ly = i.next().unwrap().parse().unwrap();
    let uy = i.next().unwrap().parse().unwrap();

    (lx, ux, ly, uy)
}

fn simulate(mut vx : i32, mut vy : i32, lx : i32, ux : i32, ly : i32, uy : i32) -> Option<i32>
{
    let mut px = 0;
    let mut py = 0;
    let mut my = py;

    while py >= ly || vy > 0
    {
        if lx <= px && px <= ux && ly <= py && py <= uy { return Some(my) }

        px += vx;
        py += vy;
        my  = my.max(py);
        vx  = (vx.abs() - 1).max(0) * vx.signum();
        vy -= 1;
    }

    None
}
