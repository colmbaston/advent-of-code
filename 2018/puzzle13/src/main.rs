use std::collections::{ HashMap, hash_map::Entry };

fn main()
{
    let (mut carts, tracks) = parse(include_str!("../input.txt"));

    // each iteration, drain the carts into the survivors map if not
    // involved in a crash; the map is keyed by cart position for
    // efficiently checking if there is already a cart at any position
    let mut survivors = HashMap::with_capacity(carts.len());
    let mut crash_log = Vec::new();

    // simulate until there is just one cart left
    while carts.len() > 1
    {
        // the carts act in the order based on their position
        carts.sort_by_key(|&Cart { position: (x, y), .. }| (y, x));
        for mut cart in carts.drain(0..)
        {
            if survivors.remove(&cart.position).is_some()
            {
                // a previous cart has crashed into this one, so log the
                // crash and don't insert this cart into the survivors map
                crash_log.push(cart.position);
            }
            else
            {
                // no previous cart has crashed into this one, so simulate
                // one step and check if it crashes into a previous cart
                cart.step(&tracks);
                match survivors.entry(cart.position)
                {
                    Entry::Occupied(e) =>
                    {
                        e.remove();
                        crash_log.push(cart.position)
                    },
                    Entry::Vacant(e) =>
                    {
                        e.insert(cart);
                    }
                }
            }
        }

        // the survivors from this iteration
        // become the carts in the next iteration
        carts.extend(survivors.drain().map(|(_, c)| c));
    }

    // part 1: the position of the first crash
    let (x, y) = crash_log[0];
    println!("{},{}", x, y);

    // part 2: the position of the last surviving cart
    let (x, y) = carts[0].position;
    println!("{},{}", x, y);
}

fn parse(s : &str) -> (Vec<Cart>, HashMap<(u32, u32), u8>)
{
    let mut carts  = Vec::new();
    let mut tracks = HashMap::new();

    for (line, y) in s.lines().zip(0 ..)
    {
        for (b, x) in line.bytes().zip(0 ..)
        {
            match b
            {
                b'|' | b'-' | b'/' | b'\\' | b'+' =>
                {
                    tracks.insert((x, y), b);
                },
                b'^' | b'v' | b'<' | b'>' =>
                {
                    let (direction, track) = match b
                    {
                        b'^' => (Direction::Up,    b'|'),
                        b'v' => (Direction::Down,  b'|'),
                        b'<' => (Direction::Left,  b'-'),
                        b'>' => (Direction::Right, b'-'),
                        _    => unreachable!()
                    };

                    tracks.insert((x, y), track);
                    carts.push(Cart { position: (x, y), direction, turn: Turn::Left });
                },
                _ => ()
            }
        }
    }

    (carts, tracks)
}

struct Cart
{
    position:  (u32, u32),
    direction: Direction,
    turn:      Turn
}

#[derive(Clone, Copy)]
enum Direction
{
    Up    = 0,
    Right = 1,
    Down  = 2,
    Left  = 3
}

#[derive(Clone, Copy)]
enum Turn
{
    Left     = 0,
    Straight = 1,
    Right    = 2
}

impl Cart
{
    fn step(&mut self, tracks : &HashMap<(u32, u32), u8>)
    {
        // change direction if necessary
        match tracks.get(&self.position)
        {
            Some(b'/') =>
            {
                self.direction = match self.direction
                {
                    Direction::Up    => Direction::Right,
                    Direction::Right => Direction::Up,
                    Direction::Down  => Direction::Left,
                    Direction::Left  => Direction::Down
                };
            },
            Some(b'\\') =>
            {
                self.direction = match self.direction
                {
                    Direction::Up    => Direction::Left,
                    Direction::Right => Direction::Down,
                    Direction::Down  => Direction::Right,
                    Direction::Left  => Direction::Up
                };
            },
            Some(b'+') => self.direction.turn(self.turn.next()),
            _ => ()
        }

        // step in current direction
        self.direction.step(&mut self.position);
    }
}

impl Direction
{
    fn turn(&mut self, turn : Turn)
    {
        *self = match ((*self as i8 + turn as i8 - 1) % 4 + 4) % 4
        {
            0 => Direction::Up,
            1 => Direction::Right,
            2 => Direction::Down,
            3 => Direction::Left,
            _ => unreachable!()
        };
    }

    fn step(self, (x, y) : &mut (u32, u32))
    {
        match self
        {
            Direction::Up    => { *y -= 1 },
            Direction::Right => { *x += 1 },
            Direction::Down  => { *y += 1 },
            Direction::Left  => { *x -= 1 }
        }
    }
}

impl Turn
{
    fn next(&mut self) -> Turn
    {
        let result = *self;

        *self = match self
        {
            Turn::Left     => Turn::Straight,
            Turn::Straight => Turn::Right,
            Turn::Right    => Turn::Left
        };

        result
    }
}
