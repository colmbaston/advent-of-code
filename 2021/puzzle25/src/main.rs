use std::collections::HashMap;

fn main()
{
    let (width, height, mut current) = Cucumber::parse(include_str!("../input.txt"));
    let mut next = HashMap::new();

    for i in 1 ..
    {
        let mut changed = false;
        for (&current_pos@(x, y), cucumber) in current.iter()
        {
            let next_pos = match cucumber
            {
                Cucumber::East =>
                {
                    let next_pos = ((x+1) % width, y);
                    if current.contains_key(&next_pos) { current_pos } else { next_pos }
                }
                Cucumber::South =>
                {
                    let next_y   = (y+1) % height;
                    let next_pos = (x, next_y);

                    match current.get(&next_pos)
                    {
                        None                  => if current.get(&((x+width-1) % width, next_y)) == Some(&Cucumber::East) { current_pos } else { next_pos },
                        Some(Cucumber::East)  => if current.contains_key(&((x+1) % width, next_y))                       { current_pos } else { next_pos },
                        Some(Cucumber::South) => current_pos
                    }
                }
            };

            if next_pos != current_pos { changed = true }
            next.insert(next_pos, cucumber.clone());
        }

        if !changed
        {
            println!("{}", i);
            break
        }

        std::mem::swap(&mut current, &mut next);
        next.clear();
    }
}

#[derive(Clone, PartialEq, Eq)]
enum Cucumber { East, South }

impl Cucumber
{
    fn parse(s : &str) -> (usize, usize, HashMap<(usize, usize), Cucumber>)
    {
        let width         = s.find('\n').unwrap();
        let height        = s.len() / (width+1);
        let mut cucumbers = HashMap::new();

        for (y, l) in s.lines().enumerate()
        {
            for (x, b) in l.bytes().enumerate()
            {
                match b
                {
                    b'>' => { cucumbers.insert((x, y), Cucumber::East);  },
                    b'v' => { cucumbers.insert((x, y), Cucumber::South); },
                    _    => ()
                }
            }
        }

        (width, height, cucumbers)
    }
}
