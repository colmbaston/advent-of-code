use std::collections::HashMap;
use aoc::direction::Direction;

fn main()
{
    let sequences = include_str!("../input.txt").lines().collect::<Vec<&str>>();
    let mut cache = HashMap::new();
    println!("{}", sequences.iter().map(|seq| complexity(seq, 2, &mut cache)).sum::<u64>());
    cache.clear();
    println!("{}", sequences.iter().map(|seq| complexity(seq, 25, &mut cache)).sum::<u64>());
}

type Pos = (i8, i8);

fn complexity(seq : &str, limit : u8, cache : &mut HashMap<(Pos, Pos, u8), u64>) -> u64
{
    let digits = seq.strip_suffix("A").unwrap();
    sequence_cost(digits.bytes(), 0, limit, cache) * digits.parse::<u64>().unwrap()
}

fn sequence_cost(seq : impl Iterator<Item = u8>, layer : u8, limit : u8, cache : &mut HashMap<(Pos, Pos, u8), u64>) -> u64
{
    let mut buttons = vec![b'A'];
    buttons.extend(seq);
    buttons.push(b'A');

    buttons.windows(2)
           .map(|w| button_cost(button_pos(w[0]), button_pos(w[1]), layer, limit, cache))
           .sum()
}

fn button_cost(from : Pos, to : Pos, layer : u8, limit : u8, cache : &mut HashMap<(Pos, Pos, u8), u64>) -> u64
{
    if let Some(&k) = cache.get(&(from, to, layer)) { return k }

    let cost = if layer == limit
    {
        (from.0.abs_diff(to.0) + from.1.abs_diff(to.1)) as u64 + 1
    }
    else
    {
        let keypad = if layer == 0 { numeric } else { directional };
        paths(from, to, keypad).into_iter()
                               .map(|path| sequence_cost(path, layer+1, limit, cache))
                               .min().unwrap()
    };

    cache.insert((from, to, layer), cost);
    cost
}

fn paths(from : Pos, to : Pos, valid : impl Fn(Pos) -> bool) -> Vec<impl Iterator<Item = u8>>
{
    let mut paths = Vec::new();
    let mut stack = vec![(from, Vec::new())];
    while let Some((pos, path)) = stack.pop()
    {
        if pos == to
        {
            paths.push(path.into_iter().map(|(_, dir)| dir_to_button(dir)));
            continue
        }

        for dir in Direction::ELEMS.into_iter()
        {
            let next = dir.step(pos);
            if valid(next) && !path.iter().any(|&(prev, _)| prev == next)
            {
                let mut path = path.clone();
                path.push((pos, dir));
                stack.push((next, path))
            }
        }
    }
    paths
}

fn button_pos(button : u8) -> Pos
{
    match button
    {
        b'A' => ( 0,  0),
        b'0' => (-1,  0),
        b'1' => (-2, -1),
        b'2' => (-1, -1),
        b'3' => ( 0, -1),
        b'4' => (-2, -2),
        b'5' => (-1, -2),
        b'6' => ( 0, -2),
        b'7' => (-2, -3),
        b'8' => (-1, -3),
        b'9' => ( 0, -3),
        b'^' => (-1,  0),
        b'<' => (-2,  1),
        b'v' => (-1,  1),
        b'>' => ( 0,  1),
        _    => unreachable!()
    }
}

fn numeric(pos : Pos) -> bool
{
    (-2 ..= 0).contains(&pos.0) && (-3 ..= 0).contains(&pos.1) && pos != (-2, 0)
}

fn directional(pos : Pos) -> bool
{
    (-2 ..= 0).contains(&pos.0) && (0 ..= 1).contains(&pos.1) && pos != (-2, 0)
}

fn dir_to_button(dir : Direction) -> u8
{
    match dir
    {
        Direction::North => b'^',
        Direction::East  => b'>',
        Direction::South => b'v',
        Direction::West  => b'<'
    }
}
