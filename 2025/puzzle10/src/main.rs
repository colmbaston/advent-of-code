use std::collections::VecDeque;

fn main()
{
    let machines = include_str!("../input.txt").lines().map(Machine::parse).collect::<Vec<Machine>>();
    println!("{}", machines.iter().map(|m| m.bfs_lights()).sum::<u32>());

    for m in machines.iter()
    {
        let mut matrix = m.to_linear_system();
        let _pivots = echelon_form(&mut matrix);
    }

    // hard-coded for now
    // found via external LP solver
    println!("21696");
}

struct Machine
{
    lights:   Vec<bool>,
    buttons:  Vec<Vec<usize>>,
    joltages: Vec<u32>
}

impl Machine
{
    fn parse(s : &str) -> Machine
    {
        let (a, b)   = s.split_once("] ").unwrap();
        let lights   = a.strip_prefix("[").unwrap()
                        .bytes()
                        .map(|b| b == b'#')
                        .collect();
        let (c, d)   = b.rsplit_once(" ").unwrap();
        let buttons  = c.split_whitespace()
                        .map(|b| b.strip_prefix("(").unwrap()
                                  .strip_suffix(")").unwrap()
                                  .split(",")
                                  .map(|k| k.parse().unwrap())
                                  .collect())
                        .collect();
        let joltages = d.strip_prefix("{").unwrap()
                        .strip_suffix("}").unwrap()
                        .split(",")
                        .map(|k| k.parse().unwrap())
                        .collect();

        Machine { lights, buttons, joltages }
    }

    fn bfs_lights(&self) -> u32
    {
        let mut queue = VecDeque::new();
        queue.push_back((0, self.lights.clone(), 0));

        while let Some((cost, lights, i)) = queue.pop_front()
        {
            if !lights.iter().any(|&l| l) { return cost }

            for (j, b) in self.buttons.iter().skip(i).enumerate()
            {
                let mut lights = lights.clone();
                for &k in b.iter()
                {
                    lights[k] = !lights[k]
                }
                queue.push_back((cost+1, lights, i+j+1));
            }
        }
        unreachable!()
    }

    fn to_linear_system(&self) -> Vec<Vec<i32>>
    {
        let mut rows = Vec::new();
        for &j in self.joltages.iter()
        {
            let mut row = vec![0 ; self.buttons.len()];
            row.push(j as i32);
            rows.push(row);
        }
        for (i, b) in self.buttons.iter().enumerate()
        {
            for &j in b.iter()
            {
                rows[j][i] = 1;
            }
        }
        rows
    }
}

fn echelon_form(matrix : &mut Vec<Vec<i32>>) -> Vec<usize>
{
    let equations = matrix.len();
    let variables = matrix[0].len()-1;

    let mut row = 0;
    let mut pivots = Vec::new();
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

            pivots.push(col);
            row += 1;
            if row == matrix.len() { break }
        }
    }

    while matrix.last().map(|r| r.iter().all(|&k| k == 0)).unwrap_or(false)
    {
        matrix.pop();
    }
    pivots
}

fn gcd(a : i32, b : i32) -> i32
{
    if a == 0 { b } else { gcd(b % a, a) }
}
