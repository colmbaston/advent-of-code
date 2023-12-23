#![feature(iter_next_chunk)]
use std::{ collections::HashMap, ops::{ Index, IndexMut }};

fn main()
{
    let mut bricks = include_str!("../input.txt").lines().map(Brick::parse).collect::<Vec<Brick>>();
    bricks.sort_unstable();
    count_falling(&mut bricks);

    let mut buffer = Vec::with_capacity(bricks.len());
    let (one, two) = (0 .. bricks.len()).fold((0, 0), |(one, two), ix|
    {
        buffer.clear();
        buffer.extend(bricks[..   ix].iter().cloned());
        buffer.extend(bricks[ix+1 ..].iter().cloned());

        let falling = count_falling(&mut buffer);
        (one + (falling == 0) as u32, two + falling)
    });

    println!("{one}");
    println!("{two}");
}

fn count_falling(bricks : &mut [Brick]) -> u32
{
    let mut count   = 0;
    let mut heights = HashMap::new();
    for brick in bricks.iter_mut()
    {
        let height = if let Axis::Z = brick.orientation
        {
            let pos = brick.min_corner;
            *heights.get(&(pos.x, pos.y)).unwrap_or(&0)
        }
        else
        {
            brick.points()
                 .map(|pos| *heights.get(&(pos.x, pos.y)).unwrap_or(&0))
                 .max().unwrap_or(0)
        };

        if brick.min_corner[Axis::Z] > height
        {
            brick.min_corner[Axis::Z] = height;
            count += 1
        }

        if let Axis::Z = brick.orientation
        {
            let pos = brick.min_corner;
            heights.insert((pos.x, pos.y), pos.z+brick.size);
        }
        else
        {
            heights.extend(brick.points()
                                .map(|pos| ((pos.x, pos.y), pos.z+1)))
        }
    }
    count
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Brick
{
    min_corner:  Pos,
    orientation: Axis,
    size:        u32
}

impl Brick
{
    fn parse(s : &str) -> Brick
    {
        let (a, b) = s.split_once('~').unwrap();
        let (a, b) = (Pos::parse(a), Pos::parse(b));
        let diff   = a.abs_diff(b);

        Brick
        {
            min_corner:  a.min(b),
            orientation:      if diff.x > 0 { Axis::X }
                         else if diff.y > 0 { Axis::Y }
                         else               { Axis::Z },
            size:        1 + diff.x + diff.y + diff.z
        }
    }

    fn points(&self) -> impl Iterator<Item = Pos> + '_
    {
        (0 .. self.size).map(|offset|
        {
            let mut pos = self.min_corner;
            pos[self.orientation] += offset;
            pos
        })
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Pos
{
    z: u32,
    x: u32,
    y: u32
}

impl Pos
{
    fn parse(s : &str) -> Pos
    {
        let [x, y, z] = s.split(',').map(|k| k.parse().unwrap()).next_chunk().unwrap();
        Pos { x, y, z }
    }

    fn abs_diff(self, other : Pos) -> Pos
    {
        Pos
        {
            x: self.x.abs_diff(other.x),
            y: self.y.abs_diff(other.y),
            z: self.z.abs_diff(other.z)
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Axis { X, Y, Z }

impl Index<Axis> for Pos
{
    type Output = u32;

    fn index(&self, axis : Axis) -> &u32
    {
        match axis
        {
            Axis::X => &self.x,
            Axis::Y => &self.y,
            Axis::Z => &self.z
        }
    }
}

impl IndexMut<Axis> for Pos
{
    fn index_mut(&mut self, axis : Axis) -> &mut u32
    {
        match axis
        {
            Axis::X => &mut self.x,
            Axis::Y => &mut self.y,
            Axis::Z => &mut self.z
        }
    }
}
