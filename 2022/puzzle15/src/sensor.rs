use std::ops::RangeInclusive;

pub struct Sensor
{
    pos:   Pos,
    range: u32
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos
{
    pub x: i32,
    pub y: i32
}

impl Sensor
{
    pub fn parse(s : &str) -> Option<(Sensor, Pos)>
    {
        let s        = s.strip_prefix("Sensor at ")?;
        let (pos, s) = s.split_at(s.find(':')?);
        let beacon   = s.strip_prefix(": closest beacon is at ")?;

        let pos    = Pos::parse(pos)?;
        let beacon = Pos::parse(beacon)?;
        Some((Sensor { pos, range: pos.manhattan(&beacon) }, beacon))
    }

    pub fn row_coverage(&self, row : i32) -> RangeInclusive<i32>
    {
        let diff = self.range as i32 - row.abs_diff(self.pos.y) as i32;
        self.pos.x - diff ..= self.pos.x + diff
    }
}

impl Pos
{
    pub fn parse(s : &str) -> Option<Pos>
    {
        let mut parts = s.split(", ");
        Some(Pos
        {
            x: parts.next()?.strip_prefix("x=")?.parse::<i32>().ok()?,
            y: parts.next()?.strip_prefix("y=")?.parse::<i32>().ok()?
        })
    }

    fn manhattan(&self, other : &Pos) -> u32
    {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}
