fn main()
{
    let (a, b)  = include_str!("../input.txt").rsplit_once("\n\n").unwrap();
    let shapes  = parse_shapes(a);
    let regions = parse_regions(b);
    println!("{}", regions.into_iter()
                          .filter(|(x, y, ps)| x*y >= ps.iter()
                                                        .zip(shapes.iter())
                                                        .map(|(s, p)| s * p).sum())
                          .count());
}

fn parse_shapes(s : &str) -> Vec<u32>
{
    s.split("\n\n")
     .map(|a| a.lines().skip(1)
               .flat_map(|l| l.bytes())
               .filter(|&b| b == b'#')
               .count() as u32)
     .collect()
}

fn parse_regions(s : &str) -> Vec<(u32, u32, Vec<u32>)>
{
    s.lines().map(|l|
    {
        let (x, a) = l.split_once('x').unwrap();
        let (y, b) = a.split_once(": ").unwrap();

        (x.parse().unwrap(),
         y.parse().unwrap(),
         b.split_whitespace()
          .map(|k| k.parse().unwrap())
          .collect())
    })
    .collect()
}
