#![feature(iter_next_chunk)]
use std::ops::RangeInclusive;
use num::{ BigInt, Zero };

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

    let mut matrix = [[0, vbz - vaz, vay - vby, 0, paz - pbz, pby - pay, vay * paz - pay * vaz + pby * vbz - vby * pbz].map(|k| k.into()).to_vec(),
                      [0, vcz - vaz, vay - vcy, 0, paz - pcz, pcy - pay, vay * paz - pay * vaz + pcy * vcz - vcy * pcz].map(|k| k.into()).to_vec(),
                      [vaz - vbz, 0, vbx - vax, pbz - paz, 0, pax - pbx, pax * vaz - vax * paz + vbx * pbz - pbx * vbz].map(|k| k.into()).to_vec(),
                      [vaz - vcz, 0, vcx - vax, pcz - paz, 0, pax - pcx, pax * vaz - vax * paz + vcx * pcz - pcx * vcz].map(|k| k.into()).to_vec(),
                      [vby - vay, vax - vbx, 0, pay - pby, pbx - pax, 0, vax * pay - pax * vay + pbx * vby - vbx * pby].map(|k| k.into()).to_vec(),
                      [vcy - vay, vax - vcx, 0, pay - pcy, pcx - pax, 0, vax * pay - pax * vay + pcx * vcy - vcx * pcy].map(|k| k.into()).to_vec()];

    println!("{}", gaussian_elimination(&mut matrix).unwrap().into_iter().take(3).sum::<BigInt>())
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

fn gaussian_elimination(matrix : &mut [Vec<BigInt>]) -> Option<Vec<BigInt>>
{
    let rows = matrix.len();
    if matrix.iter().any(|row| row.len() != rows+1) { return None }

    // convert to row echelon form
    for pivot in 0 .. rows
    {
        matrix.swap(pivot, (pivot .. rows).find(|&i| matrix[i][pivot] != BigInt::zero())?);
        let (above, below) = matrix.split_at_mut(pivot+1);

        let row_a = above.last_mut()?;
        for row_b in below.iter_mut()
        {
            let pivot_b = row_b[pivot].clone();
            if  pivot_b == BigInt::zero() { continue }
            let pivot_a = row_a[pivot].clone();

            for (a, b) in row_a[pivot..].iter_mut().zip(row_b[pivot..].iter_mut())
            {
                *a *= &pivot_b;
                *b *= &pivot_a;
                *b -= &*a;
            }
        }
    }

    // backsubstitute
    let mut solution = Vec::with_capacity(rows);
    for (pivot, row) in matrix.iter().enumerate().rev()
    {
        let (sum, coeffs) = row.split_last()?;
        let mut sum = sum.clone();

        coeffs.iter().rev().zip(solution.iter()).for_each(|(c, v)| sum -= c * v);
        solution.push(sum.checked_div(&row[pivot])?)
    }
    solution.reverse();
    Some(solution)
}
