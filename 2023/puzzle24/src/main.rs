#![feature(iter_next_chunk)]
use std::ops::RangeInclusive;

fn main()
{
    let hailstones = include_str!("../input.txt").lines().map(Hailstone::parse).collect::<Vec<Hailstone>>();

    let mut count = 0;
    for (i, ha) in hailstones.iter().enumerate()
    {
        for hb in hailstones[i+1..].iter()
        {
            if let Some((x, y)) = ha.intersect_2d(hb)
            {
                const TEST_RANGE : RangeInclusive<f64> = 200_000_000_000_000.0
                                                     ..= 400_000_000_000_000.0;

                if TEST_RANGE.contains(&x)
                && TEST_RANGE.contains(&y) { count += 1 }
            }
        }
    }
    println!("{count}");

    let Hailstone { pos: Vec3 { x: pax, y: pay, z: paz }, vel: Vec3 { x: vax, y: vay, z: vaz }} = hailstones[0];
    let Hailstone { pos: Vec3 { x: pbx, y: pby, z: pbz }, vel: Vec3 { x: vbx, y: vby, z: vbz }} = hailstones[1];
    let Hailstone { pos: Vec3 { x: pcx, y: pcy, z: pcz }, vel: Vec3 { x: vcx, y: vcy, z: vcz }} = hailstones[2];

    let mut matrix = [[0, vbz - vaz, vay - vby, 0, paz - pbz, pby - pay, vay * paz - pay * vaz + pby * vbz - vby * pbz].map(|k| k as i128).to_vec(),
                      [0, vcz - vaz, vay - vcy, 0, paz - pcz, pcy - pay, vay * paz - pay * vaz + pcy * vcz - vcy * pcz].map(|k| k as i128).to_vec(),
                      [vaz - vbz, 0, vbx - vax, pbz - paz, 0, pax - pbx, pax * vaz - vax * paz + vbx * pbz - pbx * vbz].map(|k| k as i128).to_vec(),
                      [vaz - vcz, 0, vcx - vax, pcz - paz, 0, pax - pcx, pax * vaz - vax * paz + vcx * pcz - pcx * vcz].map(|k| k as i128).to_vec(),
                      [vby - vay, vax - vbx, 0, pay - pby, pbx - pax, 0, vax * pay - pax * vay + pbx * vby - vbx * pby].map(|k| k as i128).to_vec(),
                      [vcy - vay, vax - vcx, 0, pay - pcy, pcx - pax, 0, vax * pay - pax * vay + pcx * vcy - vcx * pcy].map(|k| k as i128).to_vec()];

    println!("{}", gaussian_elimination(&mut matrix).unwrap().into_iter().take(3).sum::<i128>())
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

    fn intersect_2d(&self, other : &Hailstone) -> Option<(f64, f64)>
    {
        let Vec3 { x: pax, y: pay, .. } = self.pos;
        let Vec3 { x: vax, y: vay, .. } = self.vel;
        let Vec3 { x: pbx, y: pby, .. } = other.pos;
        let Vec3 { x: vbx, y: vby, .. } = other.vel;

        let det = vbx * vay - vax * vby;
        if let 0 = det { return None }

        let ta = (vbx * (pby - pay) - vby * (pbx - pax)) as f64 / det as f64;
        let tb = (vax * (pby - pay) - vay * (pbx - pax)) as f64 / det as f64;

        (ta >= 0.0 && tb >= 0.0).then_some((pax as f64 + vax as f64 * ta,
                                            pay as f64 + vay as f64 * ta))
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

fn gaussian_elimination(matrix : &mut [Vec<i128>]) -> Option<Vec<i128>>
{
    let equations = matrix.len();
    let variables = matrix[0].len()-1;

    // convert to row echelon form
    let mut row = 0;
    for col in 0 .. variables
    {
        if let Some(i) = (row .. equations).map(|i| (matrix[i][col].abs(), i))
                                           .filter(|(k, _)| *k != 0)
                                           .min().map(|(_, i)| i)
        {
            matrix.swap(row, i);

            let (above, below) = matrix.split_at_mut(row+1);
            let pivot_row = &mut above[row];
            let pivot_val = pivot_row[col];

            for below_row in below.iter_mut()
            {
                let below_val = below_row[col];
                if below_val == 0 { continue }

                let mut gcd_b = 0;
                for (a, b) in pivot_row[col ..].iter().zip(below_row[col ..].iter_mut())
                {
                    *b = pivot_val * *b - below_val * *a;
                    gcd_b = gcd(gcd_b, *b);
                }
                if gcd_b != 0 { below_row[col ..].iter_mut().for_each(|b| *b /= gcd_b) }
            }

            row += 1;
            if row == matrix.len() { break }
        }
    }

    // backsubstitute
    let mut solution = Vec::with_capacity(variables);
    for (pivot, row) in matrix.iter().enumerate().rev()
    {
        let (&(mut sum), coeffs) = row.split_last()?;
        for (c, v) in coeffs.iter().rev().zip(solution.iter())
        {
            sum = sum.checked_sub(c.checked_mul(*v)?)?
        }
        solution.push(sum.checked_div(row[pivot])?)
    }
    solution.reverse();
    Some(solution)
}

fn gcd(a : i128, b : i128) -> i128
{
    if let 0 = a { b } else { gcd(b % a, a) }
}
