fn main()
{
    let mut locks = Vec::new();
    let mut keys  = Vec::new();
    for (is_lock, heights) in include_str!("../input.txt").split("\n\n").map(parse_schematic)
    {
        if is_lock { locks.push(heights) } else { keys.push(heights) }
    }

    let mut count = 0;
    for lock in locks.into_iter()
    {
        for key in keys.iter()
        {
            count += lock.iter().zip(key.iter()).all(|(l, k)| l + k <= 7) as u32
        }
    }
    println!("{count}");
}

fn parse_schematic(s : &str) -> (bool, [u8 ; 5])
{
    let is_lock = s.as_bytes()[0] == b'#';
    let mut lines = s.lines().map(|l| l.as_bytes()).collect::<Vec<&[u8]>>();
    if is_lock { lines.reverse() }

    let mut heights = [0 ; 5];
    for l in lines.into_iter()
    {
        for (&b, h) in l.iter().zip(heights.iter_mut())
        {
            if b == b'#' { *h += 1 }
        }
    }

    (is_lock, heights)
}
