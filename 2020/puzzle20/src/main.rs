use std::collections::HashMap;

fn main()
{
    let mut input = include_str!("../input.txt").split("\n\n").map(parse_tile).collect::<HashMap<_, _>>();
    let mut image = HashMap::new();
    build_image(&mut image, &mut input);

    let &(min_x, min_y) = image.keys().min().unwrap();
    let &(max_x, max_y) = image.keys().max().unwrap();
    println!("{}", image.get(&(min_x, min_y)).unwrap().0
                 * image.get(&(min_x, max_y)).unwrap().0
                 * image.get(&(max_x, min_y)).unwrap().0
                 * image.get(&(max_x, max_y)).unwrap().0);
}

type Tile = Vec<Vec<bool>>;

fn parse_tile(s : &str) -> (u64, Tile)
{
    let mut it = s.lines();

    (it.next().and_then(|l| l[5 .. l.len()-1].parse().ok()).unwrap(),
     it.map(|l| l.bytes().map(|b| b == b'#').collect()).collect())
}

#[derive(Clone, Debug)]
struct Orientation
{
    reflect: bool,
    rotate:  u8
}

impl Iterator for Orientation
{
    type Item = Orientation;

    fn next(&mut self) -> Option<Orientation>
    {
        let result  = self.clone();
        self.rotate = (self.rotate + 1) % 4;
        if self.rotate == 0 { self.reflect = !self.reflect }
        Some(result)
    }
}

impl Orientation
{
    fn new() -> Orientation
    {
        Orientation { reflect: false, rotate: 0 }
    }

    fn orientations() -> impl Iterator<Item = Orientation>
    {
        Orientation::new().take(8)
    }

    fn compose(&self, other : &Orientation) -> Orientation
    {
        Orientation
        {
            reflect: self.reflect ^ other.reflect,
            rotate:  (if other.reflect { 4 - self.rotate } else { self.rotate } + other.rotate) % 4
        }
    }

    fn transform(&self, (mut x, mut y) : (usize, usize), size : usize) -> (usize, usize)
    {
        if self.reflect { x = size - x - 1 }
        for _ in 0 .. self.rotate
        {
            std::mem::swap(&mut x, &mut y);
            x = size - x - 1
        }
        (x, y)
    }
}

fn build_image(image : &mut HashMap<(i32, i32), (u64, Tile, Orientation)>, tiles : &mut HashMap<u64, Tile>)
{
    let mut queue = vec![(0, 0)];
    while let Some((x, y)) = queue.pop()
    {
        if tiles.is_empty()            {  return  }
        if image.contains_key(&(x, y)) { continue }

        let mut fitting_tile = None;
        for (id, t1, o1) in tiles.iter().flat_map(|(id, t)| Orientation::orientations().map(move |o| (id, t, o)))
        {
            let fits = image.get(&(x-1, y)).map(|(_, t2, o2)| match_left_right( t2,  o2, t1, &o1)).unwrap_or(true)
                    && image.get(&(x+1, y)).map(|(_, t2, o2)| match_left_right( t1, &o1, t2,  o2)).unwrap_or(true)
                    && image.get(&(x, y-1)).map(|(_, t2, o2)| match_above_below(t2,  o2, t1, &o1)).unwrap_or(true)
                    && image.get(&(x, y+1)).map(|(_, t2, o2)| match_above_below(t1, &o1, t2,  o2)).unwrap_or(true);

            if fits
            {
                fitting_tile = Some((*id, o1));
                break
            }
        }

        if let Some((id, o)) = fitting_tile
        {
            if let Some(t) = tiles.remove(&id)
            {
                image.insert((x, y), (id, t, o));
                queue.extend(vec![(x-1, y), (x+1, y), (x, y-1), (x, y+1)])
            }
        }
    }
}

fn match_first_row(t1 : &Tile, o1 : &Orientation, t2 : &Tile, o2 : &Orientation) -> bool
{
    let size = t1.len();

    (0 .. size).all(|x|
    {
        let (x1, y1) = o1.transform((x, 0), size);
        let (x2, y2) = o2.transform((x, 0), size);

        t1.get(y1).and_then(|r| r.get(x1)) == t2.get(y2).and_then(|r| r.get(x2))
    })
}

fn match_above_below(t1 : &Tile, o1 : &Orientation, t2 : &Tile, o2 : &Orientation) -> bool
{
    match_first_row(t1, &Orientation { reflect: true, rotate: 2 }.compose(o1),
                    t2,  o2)
}

fn match_left_right(t1 : &Tile, o1 : &Orientation, t2 : &Tile, o2 : &Orientation) -> bool
{
    match_first_row(t1, &Orientation { reflect: false, rotate: 1 }.compose(o1),
                    t2, &Orientation { reflect: true,  rotate: 3 }.compose(o2))
}
