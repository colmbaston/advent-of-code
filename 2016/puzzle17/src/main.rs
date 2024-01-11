use md5::{ Md5, Digest };
use aoc::direction::Direction;
use std::collections::VecDeque;

fn main()
{
    let input = include_str!("../input.txt").trim_end();

    let mut queue = VecDeque::new();
    queue.push_back(((0, 0), String::new()));

    while let Some((pos, path)) = queue.pop_front()
    {
        if pos == (3, 3)
        {
            println!("{path}");
            break
        }

        let mut hasher = Md5::new();
        hasher.update(input);
        hasher.update(&path);
        let hash = hasher.finalize();

        queue.extend([(Direction::North, hash[0] >>  4),
                      (Direction::South, hash[0] & 0xf),
                      (Direction::West,  hash[1] >>  4),
                      (Direction::East,  hash[1] & 0xf)].into_iter()
                                                        .filter(|(_, b)| *b > 0xa)
                                                        .filter_map(|(dir, _)| dir.checked_step(pos).map(|pos| (pos, dir)))
                                                        .filter(|((x, y), _)| *x <= 3 && *y <= 3)
                                                        .map(|(pos, dir)| (pos, { let mut path = path.clone(); path.push(format_dir(dir) as char); path })))
    }
}

fn format_dir(dir : Direction) -> u8
{
    match dir
    {
        Direction::North => b'U',
        Direction::South => b'D',
        Direction::West  => b'L',
        Direction::East  => b'R'
    }
}
