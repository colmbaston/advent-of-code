pub enum Dir { Up, Right, Down, Left }

impl Dir
{
    pub fn turn_right(&mut self)
    {
        *self = match *self
        {
            Dir::Up    => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down  => Dir::Left,
            Dir::Left  => Dir::Up
        };
    }

    pub fn turn_left(&mut self)
    {
        *self = match *self
        {
            Dir::Up    => Dir::Left,
            Dir::Left  => Dir::Down,
            Dir::Down  => Dir::Right,
            Dir::Right => Dir::Up
        };
    }

    pub fn advance(&self, (x, y) : &mut (i64, i64))
    {
        match *self
        {
            Dir::Up    => *y -= 1,
            Dir::Right => *x += 1,
            Dir::Down  => *y += 1,
            Dir::Left  => *x -= 1
        }
    }
}
