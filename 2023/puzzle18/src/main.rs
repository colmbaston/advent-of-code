use aoc::direction::Direction;

fn main()
{
    let input = include_str!("../input.txt");
    println!("{}", Polygon::parse(input, Polygon::parse_line_one).area());
    println!("{}", Polygon::parse(input, Polygon::parse_line_two).area());
}

type Pos = (i64, i64);
const ORIGIN : Pos = (0, 0);

struct Polygon(Vec<Pos>);

impl Polygon
{
    fn parse(s : &str, parse_line : impl Fn(&str) -> (Direction, u32)) -> Polygon
    {
        let mut prev = ORIGIN;
        let mut poly = vec![ORIGIN];
        for (dir, len) in s.lines().map(parse_line)
        {
            prev = dir.offset(prev, len as i64);
            poly.push(prev)
        }
        Polygon(poly)
    }

    fn parse_line_one(l : &str) -> (Direction, u32)
    {
        let dir = match l.as_bytes()[0]
        {
            b'U' => Direction::North,
            b'R' => Direction::East,
            b'D' => Direction::South,
            b'L' => Direction::West,
            _    => unreachable!()
        };

        let len = l[2..].split_at(l[2..].find(|c : char| !c.is_ascii_digit()).unwrap()).0
                        .parse().unwrap();

        (dir, len)
    }

    fn parse_line_two(l : &str) -> (Direction, u32)
    {
        let dir = match l.as_bytes()[l.len()-2]
        {
            b'0' => Direction::East,
            b'1' => Direction::South,
            b'2' => Direction::West,
            b'3' => Direction::North,
            _    => unreachable!()
        };

        let len = u32::from_str_radix(&l[l.len()-7 .. l.len()-2], 16).unwrap();

        (dir, len)
    }

    fn border(&self) -> u64
    {
        self.0.array_windows()
              .map(|&[(x1, y1), (x2, y2)]| x1.abs_diff(x2).max(y1.abs_diff(y2)))
              .sum()
    }

    fn shoelace(&self) -> u64
    {
        self.0.array_windows()
              .map(|&[(x1, y1), (x2, y2)]| (x1 - x2) * (y1 + y2))
              .sum::<i64>()
              .unsigned_abs() / 2
    }

    fn area(&self) -> u64
    {
        self.shoelace() + self.border() / 2 + 1
    }
}
