#![feature(iter_next_chunk)]
use std::collections::HashMap;

fn main()
{
    let mut parts   = Vec::new();
    let mut symbols = HashMap::new();

    for (row, y) in include_str!("../input.txt").lines().map(str::as_bytes).zip(0..)
    {
        let mut x = 0;
        while let Some(&b) = row.get(x)
        {
            if b.is_ascii_digit()
            {
                let num = row[x..].split(|b| !b.is_ascii_digit()).next().unwrap();
                parts.push(Part
                {
                    number: num.iter().fold(0, |a, k| 10 * a + (k - b'0') as u32),
                    pos:    (x as i32, y),
                    len:    num.len()
                });
                x += num.len();
            }
            else
            {
                if b != b'.' { symbols.insert((x as i32, y), b); }
                x += 1
            }
        }
    }

    println!("{}", parts.iter()
                        .filter_map(|part| part.surrounding()
                                               .any(|pos| symbols.contains_key(&pos))
                                               .then_some(part.number))
                        .sum::<u32>());

    symbols.retain(|_, b : &mut u8| *b == b'*');
    println!("{}", symbols.into_keys().filter_map(|pos|
    {
        let mut ps = parts.iter()
                          .filter(|part| adjacent(pos).any(|p| part.covers(p)));

        ps.next_chunk().ok()
          .and_then(|[p1, p2]| ps.next().is_none()
                                 .then_some(p1.number * p2.number))
    })
    .sum::<u32>());
}

type Pos = (i32, i32);

struct Part
{
    number: u32,
    pos:    Pos,
    len:    usize
}

fn adjacent((x, y) : Pos) -> impl Iterator<Item = Pos>
{
    (x-1 ..= x+1).map(move |x| (x, y-1)).chain((x-1 ..= x+1).map(move |x| (x, y+1)))
                                        .chain(std::iter::once((x-1, y)))
                                        .chain(std::iter::once((x+1, y)))
}

impl Part
{
    fn surrounding(&self) -> impl Iterator<Item = Pos>
    {
        let (x, y) = self.pos;
        let len    = self.len as i32;

        (x-1 ..= x+len).map(move |x| (x, y-1)).chain((x-1 ..= x+len).map(move |x| (x, y+1)))
                                              .chain(std::iter::once((x-1,   y)))
                                              .chain(std::iter::once((x+len, y)))
    }

    fn covers(&self, (x, y) : Pos) -> bool
    {
        let (px, py) = self.pos;

        y == py && px <= x && x < px + self.len as i32
    }
}
