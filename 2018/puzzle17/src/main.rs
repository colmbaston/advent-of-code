use std::collections::{ HashMap, hash_map::Entry };

fn main()
{
    let (min_y, max_y, mut reservoir) = parse(include_str!("../input.txt"));

    flow((500, min_y.max(0)), max_y, &mut reservoir);
    println!("{}", reservoir.values().filter(|&v| *v != Tile::Clay   ).count());
    println!("{}", reservoir.values().filter(|&v| *v == Tile::Settled).count());
}

type Reservoir = HashMap<(i32, i32), Tile>;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile
{
    Clay,
    Flowing,
    Settled
}

fn parse(s : &str) -> (i32, i32, Reservoir)
{
    fn parse_digits(s : &str) -> (i32, &str)
    {
        let (digits, s) = s.split_at(s.find(|c : char| !c.is_ascii_digit()).unwrap_or(s.len()));
        (digits.parse().unwrap(), s)
    }

    let mut min_y     = i32::MAX;
    let mut max_y     = i32::MIN;
    let mut reservoir = HashMap::new();

    for s in s.lines()
    {
        let vertical   = s.as_bytes()[0] == b'x';
        let (fixed, s) = parse_digits(&s[2..]);
        let (lower, s) = parse_digits(&s[4..]);
        let (upper, _) = parse_digits(&s[2..]);

        if vertical
        {
            min_y = min_y.min(lower);
            max_y = max_y.max(upper);
            reservoir.extend((lower ..= upper).map(|y| ((fixed, y), Tile::Clay)));
        }
        else
        {
            min_y = min_y.min(fixed);
            max_y = max_y.max(fixed);
            reservoir.extend((lower ..= upper).map(|x| ((x, fixed), Tile::Clay)));
        }
    }

    (min_y, max_y, reservoir)
}

fn flow((x, mut y) : (i32, i32), max_y : i32, reservoir : &mut Reservoir)
{
    // the flowing water falls vertically until it hits a tile
    let tile = loop
    {
        match reservoir.entry((x, y))
        {
            Entry::Vacant(e) =>
            {
                e.insert(Tile::Flowing);
                y += 1;

                // only simulate up to max_y
                if y > max_y { return }
            },
            Entry::Occupied(e) => break *e.get()
        }
    };

    // if the falling water hit a supporting tile, it fills the space
    if let Tile::Clay | Tile::Settled = tile
    {
        loop
        {
            // fill the above layers one-by-one
            // until the water flows over the edge
            y -= 1;

            // find the extent the water can flow to the left or the right
            // and whether each of these directions are blocked by clay
            let mut l = x;
            let mut r = x;
            let l_blocked = loop
            {
                // if there's no supporting tile below, flow is not blocked;
                // otherwise, if there's clay to the left, flow is blocked;
                // othwerise, the water flows one to the left and checks again
                if let None | Some(Tile::Flowing) = reservoir.get(&(l, y+1)) { break false }
                if let        Some(Tile::Clay)    = reservoir.get(&(l-1, y)) { break true  }
                l -= 1;
            };
            let r_blocked = loop
            {
                // as above, for the right instead of the left
                if let None | Some(Tile::Flowing) = reservoir.get(&(r, y+1)) { break false }
                if let        Some(Tile::Clay)    = reservoir.get(&(r+1, y)) { break true  }
                r += 1;
            };

            // the water settles if both sides are blocked, otherwise it's flowing
            let both_blocked = l_blocked && r_blocked;
            let water_state  = if both_blocked { Tile::Settled } else { Tile::Flowing };
            reservoir.extend((l ..= r).map(|x| ((x, y), water_state)));

            // if there was no tile below the left or the right,
            // respectively, recursively flow into that space
            if reservoir.get(&(l, y+1)).is_none() { flow((l, y+1), max_y, reservoir) }
            if reservoir.get(&(r, y+1)).is_none() { flow((r, y+1), max_y, reservoir) }

            // if either side wasn't blocked, stop filling
            if !both_blocked { break }
        }
    }
}
