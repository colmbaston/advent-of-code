pub fn echelon_form(matrix : &mut Vec<Vec<i32>>) -> (Vec<usize>, Vec<usize>)
{
    let equations = matrix.len();
    let variables = matrix[0].len()-1;

    let mut row = 0;
    let mut pivots = Vec::new();
    let mut free   = Vec::new();
    for col in 0 .. variables
    {
        match (row .. equations).map(|i| (matrix[i][col].abs(), i))
                                .filter(|(k, _)| *k != 0)
                                .min().map(|(_, i)| i)
        {
            None    => free.push(col),
            Some(i) =>
            {
                pivots.push(col);
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
                if row == matrix.len()
                {
                    free.extend(col+1 .. variables);
                    break
                }
            }
        }
    }

    while matrix.last().map(|r| r.iter().all(|&k| k == 0)).unwrap_or(false)
    {
        matrix.pop();
    }

    (pivots, free)
}

pub fn back_substitute(matrix : &[Vec<i32>], pivots : &[usize], vars : &mut [i32]) -> bool
{
    for (row, &pivot) in matrix.iter().zip(pivots.iter()).rev()
    {
        let (&(mut sum), coeffs) = row.split_last().unwrap();
        for (&c, &v) in coeffs[pivot+1 ..].iter().zip(vars[pivot+1 ..].iter())
        {
            sum -= c * v;
        }

        let quotient  = sum / row[pivot];
        let remainder = sum % row[pivot];
        if quotient < 0 || remainder != 0 { return false }
        vars[pivot] = quotient;
    }
    true
}

fn gcd(a : i32, b : i32) -> i32
{
    if a == 0 { b } else { gcd(b % a, a) }
}
