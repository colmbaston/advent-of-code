fn main()
{
    let mut input = include_str!("../input.txt").lines().map(parse_row).collect::<Vec<Reindeer>>();

    for _ in 0 .. 2503
    {
        let max_position = input.iter_mut().map(|r| { r.step(); r.position }).max().unwrap();
        input.iter_mut().filter(|r| r.position == max_position).for_each(|r| r.points += 1)
    }

    let (position, points) = input.iter().fold((u32::MIN, u32::MIN), |(a, b), r| (a.max(r.position), b.max(r.points)));
    println!("{}", position);
    println!("{}", points);
}

struct Reindeer
{
    speed:        u32,
    flying_time:  u32,
    resting_time: u32,
    flying:       bool,
    countdown:    u32,
    position:     u32,
    points:       u32
}

impl Reindeer
{
    fn step(&mut self)
    {
        if let 0 = self.countdown
        {
            self.flying    = !self.flying;
            self.countdown = if self.flying { self.flying_time } else { self.resting_time };
        }

        if self.flying { self.position += self.speed }
        self.countdown -= 1;
    }
}

fn parse_row(s : &str) -> Reindeer
{
    match s.split(' ').collect::<Vec<&str>>()[..]
    {
        [_, "can", "fly", b, "km/s", "for", c, "seconds,", "but", "then", "must", "rest", "for", d, "seconds."] =>
        {
            Reindeer
            {
                speed:        b.parse().unwrap(),
                flying_time:  c.parse().unwrap(),
                resting_time: d.parse().unwrap(),
                flying:       false,
                countdown:    0,
                position:     0,
                points:       0
            }
        }
        _ => unreachable!()
    }
}
