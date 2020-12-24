use std::collections::HashSet;

fn main()
{
    let input = include_str!("../input.txt").lines().map(Axial::parse).collect::<Vec<_>>();

    let mut black = HashSet::new();
    for c in input.iter()
    {
        if black.contains(c)
        {
            black.remove(c);
        }
        else
        {
            black.insert(c.clone());
        }
    }
    println!("{}", black.len());

    for _ in 0 .. 100
    {
        black = step(&black);
    }
    println!("{}", black.len());
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Axial
{
    x: i32,
    y: i32
}

impl Axial
{
    fn parse(mut s : &str) -> Axial
    {
        let mut axial = Axial { x: 0, y: 0 };

        while !s.is_empty()
        {
            if let Some(t) = s.strip_prefix('e')  { axial.x += 1; axial.y -= 1; s = t; continue }
            if let Some(t) = s.strip_prefix("se") {               axial.y -= 1; s = t; continue }
            if let Some(t) = s.strip_prefix("sw") { axial.x -= 1;               s = t; continue }
            if let Some(t) = s.strip_prefix('w')  { axial.x -= 1; axial.y += 1; s = t; continue }
            if let Some(t) = s.strip_prefix("nw") {               axial.y += 1; s = t; continue }
            if let Some(t) = s.strip_prefix("ne") { axial.x += 1;               s = t; continue }

            unreachable!()
        }

        axial
    }

    fn adjacents(&self) -> impl Iterator<Item = Axial>
    {
        vec![Axial { x: self.x+1, y: self.y-1 },
             Axial { x: self.x,   y: self.y-1 },
             Axial { x: self.x-1, y: self.y   },
             Axial { x: self.x-1, y: self.y+1 },
             Axial { x: self.x,   y: self.y+1 },
             Axial { x: self.x+1, y: self.y   }].into_iter()
    }
}

fn step(black : &HashSet<Axial>) -> HashSet<Axial>
{
    let mut new = black.iter()
                       .flat_map(|c| std::iter::once(c.clone()).chain(c.adjacents()))
                       .collect::<HashSet<Axial>>();

    new.retain(|c|
    {
        let count = c.adjacents().filter(|d| black.contains(d)).count();
        count == 2 || count == 1 && black.contains(c)
    });

    new
}
