use aoc::direction::Direction;

fn main()
{
    let insts = include_str!("../input.txt").lines()
                                            .map(|l| l.bytes().map(parse_dir).collect())
                                            .collect::<Vec<Vec<Direction>>>();

    let keypad_one = [b"123",
                      b"456",
                      b"789"].map(|a| a.as_slice());

    let keypad_two = [b"  1  ",
                      b" 234 ",
                      b"56789",
                      b" ABC ",
                      b"  D  "].map(|a| a.as_slice());

    for (mut pos, keypad) in [((1, 1), keypad_one.as_slice()),
                              ((2, 0), keypad_two.as_slice())]
    {
        for inst in insts.iter()
        {
            for dir in inst.iter()
            {
                pos = dir.checked_step(pos)
                         .filter(|&(x, y)| keypad.get(y)
                                                 .and_then(|s| s.get(x))
                                                 .filter(|b| b.is_ascii_hexdigit())
                                                 .is_some())
                         .unwrap_or(pos)
            }
            print!("{}", keypad[pos.1][pos.0] as char);
        }
        println!();
    }
}

fn parse_dir(b : u8) -> Direction
{
    match b
    {
        b'U' => Direction::North,
        b'D' => Direction::South,
        b'L' => Direction::West,
        b'R' => Direction::East,
        _    => unreachable!()
    }
}
