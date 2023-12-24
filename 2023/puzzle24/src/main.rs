#![feature(iter_next_chunk)]

fn main()
{
    let hailstones = include_str!("../input.txt").lines().map(Hailstone::parse).collect::<Vec<Hailstone>>();

    let mut count = 0;
    for (i, ha) in hailstones.iter().enumerate()
    {
        for hb in hailstones[i+1..].iter()
        {
            const TEST_LOWER : f64 = 200_000_000_000_000.0;
            const TEST_UPPER : f64 = 400_000_000_000_000.0;

            if let Some((x, y)) = ha.intersect_2d(hb)
            {
                if (TEST_LOWER ..= TEST_UPPER).contains(&x)
                && (TEST_LOWER ..= TEST_UPPER).contains(&y)
                && (x - ha.pos.x as f64).signum() == (ha.vel.x as f64).signum()
                && (y - ha.pos.y as f64).signum() == (ha.vel.y as f64).signum()
                && (x - hb.pos.x as f64).signum() == (hb.vel.x as f64).signum()
                && (y - hb.pos.y as f64).signum() == (hb.vel.y as f64).signum()
                {
                    count += 1
                }
            }
        }
    }
    println!("{count}");

    println!("{}", solve(&hailstones).unwrap());
}

fn solve(hailstones : &[Hailstone]) -> Option<i64>
{
    use z3::{ Context, Config, Solver, ast::{ Ast, Int }, SatResult };

    let context = Context::new(&Config::new());
    let solver  = Solver::new(&context);

    let rpx = Int::new_const(&context, "rpx");
    let rpy = Int::new_const(&context, "rpy");
    let rpz = Int::new_const(&context, "rpz");
    let rvx = Int::new_const(&context, "rvx");
    let rvy = Int::new_const(&context, "rvy");
    let rvz = Int::new_const(&context, "rvz");

    for (i, hailstone) in hailstones.iter().enumerate()
    {
        let t = Int::new_const(&context, format!("t{i}"));
        solver.assert(&t.ge(&Int::from_i64(&context, 0)));

        let hpx = Int::from_i64(&context, hailstone.pos.x);
        let hpy = Int::from_i64(&context, hailstone.pos.y);
        let hpz = Int::from_i64(&context, hailstone.pos.z);
        let hvx = Int::from_i64(&context, hailstone.vel.x);
        let hvy = Int::from_i64(&context, hailstone.vel.y);
        let hvz = Int::from_i64(&context, hailstone.vel.z);
        solver.assert(&(&hpx + &hvx * &t)._eq(&(&rpx + &rvx * &t)));
        solver.assert(&(&hpy + &hvy * &t)._eq(&(&rpy + &rvy * &t)));
        solver.assert(&(&hpz + &hvz * &t)._eq(&(&rpz + &rvz * &t)));
    }

    (solver.check() == SatResult::Sat).then(|| solver.get_model()?
                                                     .eval(&(&rpx + &rpy + &rpz), true)?
                                                     .as_i64())
                                      .flatten()
}

struct Hailstone
{
    pos: Vec3,
    vel: Vec3
}

struct Vec3
{
    x: i64,
    y: i64,
    z: i64
}

impl Hailstone
{
    fn parse(s : &str) -> Hailstone
    {
        let (a, b) = s.split_once('@').unwrap();
        Hailstone { pos: Vec3::parse(a), vel: Vec3::parse(b) }
    }

    fn gradient_2d(&self) -> Option<f64>
    {
        (self.vel.x != 0).then(|| self.vel.y as f64 / self.vel.x as f64)
    }

    fn intercept_2d(&self) -> Option<f64>
    {
        self.gradient_2d().map(|g| self.pos.y as f64 - g * self.pos.x as f64)
    }

    fn intersect_2d(&self, other : &Hailstone) -> Option<(f64, f64)>
    {
        // y = ax + c
        let a = self.gradient_2d()?;
        let c = self.intercept_2d()?;

        // y = bx + d
        let b = other.gradient_2d()?;
        let d = other.intercept_2d()?;

        let x = (a != b).then(|| (d - c) / (a - b))?;
        let y = a * x + c;

        Some((x, y))
    }
}

impl Vec3
{
    fn parse(s : &str) -> Vec3
    {
        let [x, y, z] = s.split(',')
                         .map(|k| k.trim().parse().unwrap())
                         .next_chunk().unwrap();

        Vec3 { x, y, z }
    }
}
