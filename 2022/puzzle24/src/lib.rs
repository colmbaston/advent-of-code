pub fn lcm(a : i16, b : i16) -> i16
{
    (a * b) / gcd(a, b)
}

pub fn gcd(a : i16, b : i16) -> i16
{
    if b == 0 { a } else { gcd(b, a % b) }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pos
{
    pub x: i16,
    pub y: i16
}

impl Pos
{
    pub fn moves(self) -> impl Iterator<Item = Pos>
    {
        [Pos { x: self.x,   y: self.y   },
         Pos { x: self.x-1, y: self.y   },
         Pos { x: self.x+1, y: self.y   },
         Pos { x: self.x,   y: self.y-1 },
         Pos { x: self.x,   y: self.y+1 }].into_iter()
    }

    pub fn manhattan(self, other : Pos) -> u16
    {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[derive(Clone, Copy)]
pub struct Blizzard
{
    pub pos: Pos,
        dir: Direction
}

#[derive(Clone, Copy)]
enum Direction { Up, Down, Left, Right }

impl Blizzard
{
    pub fn step(&mut self, width : i16, height : i16)
    {
        match self.dir
        {
            Direction::Up    => self.pos.y = (self.pos.y - 1).rem_euclid(height),
            Direction::Down  => self.pos.y = (self.pos.y + 1) %          height,
            Direction::Left  => self.pos.x = (self.pos.x - 1).rem_euclid(width),
            Direction::Right => self.pos.x = (self.pos.x + 1) %          width
        }
    }

    pub fn parse(s : &str) -> (i16, i16, Vec<Blizzard>)
    {
        let mut width     = 0;
        let mut height    = 0;
        let mut blizzards = Vec::new();

        for (l, y) in s.lines().skip(1).zip(0 ..)
        {
            for (b, x) in l.bytes().skip(1).zip(0 ..)
            {
                let dir = match b
                {
                    b'#' => break,
                    b'^' => Some(Direction::Up),
                    b'v' => Some(Direction::Down),
                    b'<' => Some(Direction::Left),
                    b'>' => Some(Direction::Right),
                    _    => None
                };

                width  = width.max(x+1);
                height = height.max(y+1);

                if let Some(dir) = dir
                {
                    blizzards.push(Blizzard { pos: Pos { x, y }, dir });
                }
            }
        }

        (width, height, blizzards)
    }
}
