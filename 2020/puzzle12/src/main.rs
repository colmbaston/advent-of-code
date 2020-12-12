fn main()
{
    let input = parse_instructions(include_str!("../input.txt"));

    let mut ship_one = Ship::new();
    let mut ship_two = Ship::new();

    for i in input.iter()
    {
        ship_one.step_one(i);
        ship_two.step_two(i);
    }

    println!("{}", { let (x, y) = ship_one.position; x.abs() + y.abs() });
    println!("{}", { let (x, y) = ship_two.position; x.abs() + y.abs() });
}

enum Instruction
{
    N(i32),
    S(i32),
    E(i32),
    W(i32),
    L(i32),
    R(i32),
    F(i32)
}

fn parse_instructions(s : &str) -> Vec<Instruction>
{
    s.lines().map(|l|
    {
        let k = l[1..].parse().unwrap();
        match l.as_bytes()[0]
        {
            b'N' => Instruction::N(k),
            b'S' => Instruction::S(k),
            b'E' => Instruction::E(k),
            b'W' => Instruction::W(k),
            b'L' => Instruction::L(k / 90 % 4),
            b'R' => Instruction::R(k / 90 % 4),
            b'F' => Instruction::F(k),
            _    => unreachable!()
        }
    })
    .collect()
}

struct Ship
{
    position  : (i32, i32),
    direction : (i32, i32),
    waypoint  : (i32, i32)
}

fn rotate((x, y) : (i32, i32)) -> (i32, i32)
{
    (y, -x)
}

impl Ship
{
    fn new() -> Ship
    {
        Ship
        {
            position:  (0,  0),
            direction: (1,  0),
            waypoint:  (10, 1)
        }
    }

    fn step_one(&mut self, i : &Instruction)
    {
        let (px, py) = self.position;
        let (dx, dy) = self.direction;

        match i
        {
            Instruction::N(k) => self.position = (px, py + k),
            Instruction::S(k) => self.position = (px, py - k),
            Instruction::E(k) => self.position = (px + k, py),
            Instruction::W(k) => self.position = (px - k, py),
            Instruction::L(k) => for _ in 0 .. 4 - *k { self.direction = rotate(self.direction) },
            Instruction::R(k) => for _ in 0 ..     *k { self.direction = rotate(self.direction) },
            Instruction::F(k) => self.position = (px + dx * k, py + dy * k)
        }
    }

    fn step_two(&mut self, i : &Instruction)
    {
        let (px, py) = self.position;
        let (wx, wy) = self.waypoint;

        match i
        {
            Instruction::N(k) => self.waypoint = (wx, wy + k),
            Instruction::S(k) => self.waypoint = (wx, wy - k),
            Instruction::E(k) => self.waypoint = (wx + k, wy),
            Instruction::W(k) => self.waypoint = (wx - k, wy),
            Instruction::L(k) => for _ in 0 .. 4 - *k { self.waypoint = rotate(self.waypoint) },
            Instruction::R(k) => for _ in 0 ..     *k { self.waypoint = rotate(self.waypoint) },
            Instruction::F(k) => self.position = (px + wx * k, py + wy * k)
        }
    }
}
