fn main()
{
    let input = include_str!("../input.txt").lines().map(parse_dims).collect::<Vec<_>>();

    let (one, two) = input.iter().fold((0, 0), |(p, r), x| { let (q, s) = solve(x); (p+q, r+s) });
    println!("{}", one);
    println!("{}", two);
}

fn parse_dims(s : &str) -> (u32, u32, u32)
{
    let (l, s) = span_digit(s);
    let (w, s) = span_digit(&s[1..]);
    let (h, _) = span_digit(&s[1..]);

    (l.parse().unwrap(),
     w.parse().unwrap(),
     h.parse().unwrap())
}

fn span_digit(s : &str) -> (&str, &str)
{
    s.split_at(s.find(|c : char| !c.is_ascii_digit()).unwrap_or_else(|| s.len()))
}

fn solve(&(l, w, h) : &(u32, u32, u32)) -> (u32, u32)
{
    let a = l*w;
    let b = w*h;
    let c = h*l;

    let paper  = 2 * (a+b+c) + a.min(b).min(c);
    let ribbon = 2 * (l+w+h - l.max(w).max(h)) + l*w*h;

    (paper, ribbon)
}
