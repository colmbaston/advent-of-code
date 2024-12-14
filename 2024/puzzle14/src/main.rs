use std::cmp::Ordering;

fn main()
{
    let robots = include_str!("../input.txt").lines().map(Robot::parse).collect::<Vec<Robot>>();

    let mut quadrants = [0 ; 4];
    for r in robots.iter()
    {
        match (r.step_x(100).cmp(&(WIDTH/2)), r.step_y(100).cmp(&(HEIGHT/2)))
        {
            (Ordering::Less,    Ordering::Less)    => quadrants[0] += 1,
            (Ordering::Less,    Ordering::Greater) => quadrants[1] += 1,
            (Ordering::Greater, Ordering::Less)    => quadrants[2] += 1,
            (Ordering::Greater, Ordering::Greater) => quadrants[3] += 1,
            _                                      => ()
        }
    }
    println!("{}", quadrants.iter().product::<u32>());

    let mut population = Vec::with_capacity(robots.len());
    let tx = (0 .. WIDTH).min_by_key(|&t|
    {
        population.clear();
        population.extend(robots.iter().map(|r| r.step_x(t) as f32));
        variance(&population).to_bits()
    })
    .unwrap();

    let ty = (0 .. HEIGHT).min_by_key(|&t|
    {
        population.clear();
        population.extend(robots.iter().map(|r| r.step_y(t) as f32));
        variance(&population).to_bits()
    })
    .unwrap();

    println!("{}", (tx ..).step_by(WIDTH as usize).find(|&t| t % HEIGHT == ty).unwrap());
}

const WIDTH  : i32 = 101;
const HEIGHT : i32 = 103;

type Pos = (i32, i32);

struct Robot
{
    pos : Pos,
    vel : Pos
}

impl Robot
{
    fn parse(s : &str) -> Robot
    {
        let       s  = s.strip_prefix("p=").unwrap();
        let (px,  s) = s.split_once(',').unwrap();
        let (py,  s) = s.split_once(" v=").unwrap();
        let (vx, vy) = s.split_once(',').unwrap();

        Robot
        {
            pos: (px.parse().unwrap(), py.parse().unwrap()),
            vel: (vx.parse().unwrap(), vy.parse().unwrap())
        }
    }

    fn step_x(&self, t : i32) -> i32
    {
        (self.pos.0 + t * self.vel.0).rem_euclid(WIDTH)
    }

    fn step_y(&self, t : i32) -> i32
    {
        (self.pos.1 + t * self.vel.1).rem_euclid(HEIGHT)
    }
}

fn variance(population : &[f32]) -> f32
{
    let size = population.len() as f32;
    let mean = population.iter().sum::<f32>() / size;
    population.iter().map(|&k| (k - mean).powi(2)).sum::<f32>() / size
}
