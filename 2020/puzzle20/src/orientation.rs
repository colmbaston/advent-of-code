#[derive(Clone)]
pub struct Orientation
{
    pub reflect: bool,
    pub rotate:  u8
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
    pub fn new() -> Orientation
    {
        Orientation { reflect: false, rotate: 0 }
    }

    pub fn compose(&self, other : &Orientation) -> Orientation
    {
        Orientation
        {
            reflect: self.reflect ^ other.reflect,
            rotate:  (if other.reflect { 4 - self.rotate } else { self.rotate } + other.rotate) % 4
        }
    }

    pub fn transform(&self, (mut x, mut y) : (usize, usize), size : usize) -> (usize, usize)
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
