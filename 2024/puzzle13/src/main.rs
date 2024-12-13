fn main()
{
    let mut machines = include_str!("../input.txt").split("\n\n")
                                                   .map(Machine::parse)
                                                   .collect::<Vec<Machine>>();

    println!("{}", machines.iter().filter_map(Machine::min).sum::<i64>());
    machines.iter_mut().for_each(|m| { m.prize.0 += 10_000_000_000_000; m.prize.1 += 10_000_000_000_000 });
    println!("{}", machines.iter().filter_map(Machine::min).sum::<i64>());
}

type Pos = (i64, i64);

struct Machine
{
    button_a : Pos,
    button_b : Pos,
    prize    : Pos
}

impl Machine
{
    fn parse(s : &str) -> Machine
    {
        let       s  = s.strip_prefix("Button A: X+").unwrap();
        let (ax,  s) = s.split_once(", Y+").unwrap();
        let (ay,  s) = s.split_once("\nButton B: X+").unwrap();
        let (bx,  s) = s.split_once(", Y+").unwrap();
        let (by,  s) = s.split_once("\nPrize: X=").unwrap();
        let (px,  s) = s.split_once(", Y=").unwrap();
        let  py      = s.trim_end();

        Machine
        {
            button_a: (ax.parse().unwrap(), ay.parse().unwrap()),
            button_b: (bx.parse().unwrap(), by.parse().unwrap()),
            prize:    (px.parse().unwrap(), py.parse().unwrap())
        }
    }

    fn min(&self) -> Option<i64>
    {
        let (ax, ay) = self.button_a;
        let (bx, by) = self.button_b;
        let (px, py) = self.prize;

        let a = (bx * py - by * px) / (ay * bx - ax * by);
        let b = (px - a * ax) / bx;

        (a * ax + b * bx == px && a * ay + b * by == py).then_some(3*a + b)
    }
}
