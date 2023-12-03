#![feature(iter_next_chunk)]
use std::collections::HashMap;

fn main()
{
    let mut parts   = Vec::new();
    let mut symbols = HashMap::new();

    for (row, y) in include_str!("../input.txt").lines().map(str::as_bytes).zip(0..)
    {
        let mut x = 0;
        while let Some(&b) = row.get(x as usize)
        {
            if b.is_ascii_digit()
            {
                let digits = row[x as usize ..].split(|b| !b.is_ascii_digit()).next().unwrap();

                let number = digits.iter().fold(0, |a, k| 10 * a + (k - b'0') as u32);
                let pos    = (x, y);
                let len    = digits.len() as i32;

                parts.push(Part { number, pos, len });
                x += len;
            }
            else
            {
                if b != b'.' { symbols.insert((x, y), b); }
                x += 1
            }
        }
    }

    println!("{}", parts.iter()
                        .filter_map(|part| part.adjacents()
                                               .any(|pos| symbols.contains_key(&pos))
                                               .then_some(part.number))
                        .sum::<u32>());

    println!("{}", symbols.into_iter().filter_map(|(pos, symb)|
    {
        (symb == b'*').then(||
        {
            let mut ps = parts.iter()
                              .filter(|part| part.is_adjacent(pos));

            ps.next_chunk().ok()
              .and_then(|[p1, p2]| ps.next().is_none()
                                     .then_some(p1.number * p2.number))
        })
        .flatten()
    })
    .sum::<u32>());
}

type Pos = (i32, i32);

struct Part
{
    number: u32,
    pos:    Pos,
    len:    i32
}

impl Part
{
    fn adjacents(&self) -> impl Iterator<Item = Pos>
    {
        let (x, y) = self.pos;

        (x-1 ..= x+self.len).map(move |x| (x, y-1)).chain((x-1 ..= x+self.len).map(move |x| (x, y+1)))
                                                   .chain(std::iter::once((x-1,   y)))
                                                   .chain(std::iter::once((x+self.len, y)))
    }

    fn is_adjacent(&self, (x, y) : Pos) -> bool
    {
        let (px, py) = self.pos;

        (y == py-1 || y == py+1) && px-1 <= x && x <= px+self.len || y == py && (x == px-1 || x == px+self.len)
    }
}
