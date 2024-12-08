use std::collections::{ HashMap, HashSet };

fn main()
{
    let (bounds@(width, height), antennae) = parse_grid(include_str!("../input.txt"));

    println!("{}", antennae.values()
                           .flat_map(|ps| antinodes_one(ps))
                           .filter(|(x, y)| (0 .. width).contains(x) && (0 .. height).contains(y))
                           .collect::<HashSet<Pos>>().len());

    println!("{}", antennae.values()
                           .flat_map(|ps| antinodes_two(ps, bounds))
                           .collect::<HashSet<Pos>>().len());
}

type Pos = (i32, i32);

fn parse_grid(s : &str) -> (Pos, HashMap<u8, Vec<Pos>>)
{
    let mut width    = 0;
    let mut height   = 0;
    let mut antennae = HashMap::new();

    for (l, y) in s.lines().zip(0 ..)
    {
        for (b, x) in l.bytes().zip(0 ..)
        {
            if b != b'.'
            {
                antennae.entry(b).or_insert(Vec::new()).push((x, y))
            }
            width = width.max(x+1);
        }
        height = height.max(y+1);
    }

    ((width, height), antennae)
}

fn antinodes_one(antennae : &[Pos]) -> impl Iterator<Item = Pos> + '_
{
    antennae.iter().enumerate()
            .flat_map(|(i, &(x1, y1))| antennae.iter().skip(i+1)
                                               .flat_map(move |&(x2, y2)| [(x1 - x2 + x1, y1 - y2 + y1),
                                                                           (x2 + x2 - x1, y2 + y2 - y1)]))
}

fn antinodes_two(antennae : &[Pos], bounds : Pos) -> impl Iterator<Item = Pos> + '_
{
    antennae.iter().enumerate()
            .flat_map(move |(i, &p)| antennae.iter().skip(i+1)
                                             .flat_map(move |&q| line(p, q, bounds)))
}

fn line((x1, y1) : Pos, (x2, y2) : Pos, (width, height) : Pos) -> impl Iterator<Item = Pos>
{
    let dx = x2 - x1;
    let dy = y2 - y1;

    (0 ..).map(move |k| (x1 - k*dx, y1 - k*dy))
          .take_while(move |(x, y)| (0 .. width).contains(x) && (0 .. height).contains(y))
          .chain((1 ..).map(move |k| (x1 + k*dx, y1 + k*dy))
                       .take_while(move |(x, y)| (0 .. width).contains(x) && (0 .. height).contains(y)))
}
