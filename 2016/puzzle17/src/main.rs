use md5::{ Md5, Digest };
use aoc::direction::Direction;

fn main()
{
    let mut path     = Vec::new();
    let mut shortest = None;
    let mut longest  = 0;
    dfs(include_str!("../input.txt").trim_end(), (0, 0), &mut path, &mut shortest, &mut longest);
    println!("{}", std::str::from_utf8(&shortest.unwrap()).unwrap());
    println!("{longest}");
}

type Pos = (u8, u8);

fn dfs(input : &str, pos : Pos, path : &mut Vec<u8>, shortest : &mut Option<Vec<u8>>, longest : &mut usize)
{
    if pos == (3, 3)
    {
        *longest = path.len().max(*longest);
        if shortest.as_ref().filter(|s| s.len() <= path.len()).is_none() { *shortest = Some(path.clone()) }
    }
    else
    {
        let mut hasher = Md5::new();
        hasher.update(input);
        hasher.update(&path);
        let hash = hasher.finalize();

        for (dir, pos) in Direction::ELEMS.into_iter()
                                          .zip([hash[0] >> 4, hash[1] & 0xf, hash[0] & 0xf, hash[1] >> 4])
                                          .filter(|(_, b)| *b > 0xa)
                                          .filter_map(|(dir, _)| dir.checked_step(pos)
                                                                    .filter(|&(x, y)| x <= 3 && y <= 3)
                                                                    .map(|pos| (dir, pos)))
        {
            path.push(udlr(dir));
            dfs(input, pos, path, shortest, longest);
            path.pop();
        }
    }
}

fn udlr(dir : Direction) -> u8
{
    match dir
    {
        Direction::North => b'U',
        Direction::South => b'D',
        Direction::West  => b'L',
        Direction::East  => b'R'
    }
}
