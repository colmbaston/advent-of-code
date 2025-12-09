#![feature(array_windows)]

fn main()
{
    let polygon     = Polygon::parse(include_str!("../input.txt"));
    let mut max_one = 0;
    let mut max_two = 0;
    for (tl, br) in polygon.rects()
    {
        let area = (1+br.0-tl.0)*(1+br.1-tl.1);
        max_one  = max_one.max(area);

        if !polygon.corners_in_rect(tl, br)
        && !polygon.cuts_rect(tl, br)
        &&  polygon.contains(((tl.0+br.0)/2, (tl.1+br.1)/2))
        {
            max_two = max_two.max(area);
        }
    }
    println!("{max_one}");
    println!("{max_two}");
}

type Point = (u64, u64);
struct Polygon { corners: Vec<Point> }

impl Polygon
{
    fn parse(s : &str) -> Polygon
    {
        Polygon
        {
            corners: s.lines()
                      .map(|l| { let (a, b) = l.split_once(',').unwrap();
                                 (a.parse().unwrap(), b.parse().unwrap()) })
                      .collect::<Vec<Point>>()
        }
    }

    fn rects(&self) -> impl Iterator<Item = (Point, Point)>
    {
        self.corners.iter().enumerate().flat_map(|(i, p)|
        {
            self.corners.iter().skip(i+1).map(|q|
            {
                ((p.0.min(q.0), p.1.min(q.1)),
                 (p.0.max(q.0), p.1.max(q.1)))
            })
        })
    }

    fn edges(&self) -> impl Iterator<Item = (Point, Point)>
    {
        self.corners.array_windows().cloned()
                    .map(|[a, b]| (a, b))
                    .chain(std::iter::once((self.corners[self.corners.len()-1], self.corners[0])))
    }

    fn corners_in_rect(&self, tl : Point, br : Point) -> bool
    {
        self.corners.iter().any(|p| (tl.0+1 .. br.0).contains(&p.0)
                                 && (tl.1+1 .. br.1).contains(&p.1))
    }

    fn cuts_rect(&self, tl : Point, br : Point) -> bool
    {
        self.edges().any(|(mut a, mut b)|
        {
            if a > b { std::mem::swap(&mut a, &mut b) }
            a.0 == b.0 && (tl.0+1 .. br.0).contains(&a.0) && a.1 <= tl.1 && br.1 <= b.1 ||
            a.1 == b.1 && (tl.1+1 .. br.1).contains(&a.1) && a.0 <= tl.0 && br.0 <= b.0
        })
    }

    fn contains(&self, (x, y) : Point) -> bool
    {
        let mut winding = 0;
        for (a, b) in self.edges()
        {
            if y > a.1
            {
                if a.0 < b.0 && (a.0 .. b.0).contains(&x)
                {
                    winding += 1;
                }
                else if (b.0 .. a.0).contains(&x)
                {
                    winding -= 1;
                }
            }
        }
        winding != 0 || self.edge_contains((x, y))
    }

    fn edge_contains(&self, (x, y) : Point) -> bool
    {
        self.edges().any(|(mut a, mut b)|
        {
            if a > b { std::mem::swap(&mut a, &mut b) }
            x == a.0 && x == b.0 && (a.1 ..= b.1).contains(&y) ||
            y == a.1 && y == b.1 && (a.0 ..= b.0).contains(&x)
        })
    }
}
