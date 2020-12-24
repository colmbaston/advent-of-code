use std::collections::HashSet;

fn main()
{
    let input = include_str!("../input.txt").lines().map(Cube::parse).collect::<Vec<_>>();

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
struct Cube
{
    x: i32,
    y: i32,
    z: i32
}

impl Cube
{
    fn parse(mut s : &str) -> Cube
    {
        let mut cube = Cube { x: 0, y: 0, z: 0 };

        while !s.is_empty()
        {
            if let Some(t) = s.strip_prefix('e')  { cube.x += 1; cube.y -= 1; s = t; continue }
            if let Some(t) = s.strip_prefix("se") { cube.z += 1; cube.y -= 1; s = t; continue }
            if let Some(t) = s.strip_prefix("sw") { cube.z += 1; cube.x -= 1; s = t; continue }
            if let Some(t) = s.strip_prefix('w')  { cube.y += 1; cube.x -= 1; s = t; continue }
            if let Some(t) = s.strip_prefix("nw") { cube.y += 1; cube.z -= 1; s = t; continue }
            if let Some(t) = s.strip_prefix("ne") { cube.x += 1; cube.z -= 1; s = t; continue }

            unreachable!()
        }

        cube
    }

    fn adjacents(&self) -> impl Iterator<Item = Cube>
    {
        vec![Cube { x: self.x+1, y: self.y-1, z: self.z   },
             Cube { x: self.x,   y: self.y-1, z: self.z+1 },
             Cube { x: self.x-1, y: self.y,   z: self.z+1 },
             Cube { x: self.x-1, y: self.y+1, z: self.z   },
             Cube { x: self.x,   y: self.y+1, z: self.z-1 },
             Cube { x: self.x+1, y: self.y,   z: self.z-1 }].into_iter()
    }
}

fn step(black : &HashSet<Cube>) -> HashSet<Cube>
{
    let mut new = black.iter()
                       .flat_map(|c| std::iter::once(c.clone()).chain(c.adjacents()))
                       .collect::<HashSet<Cube>>();

    new.retain(|c|
    {
        let count = c.adjacents().filter(|d| black.contains(d)).count();
        if black.contains(c) { count == 1 || count == 2 } else { count == 2 }
    });

    new
}
