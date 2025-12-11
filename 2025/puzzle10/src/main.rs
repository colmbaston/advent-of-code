mod linear;
mod counter;
use std::collections::VecDeque;

fn main()
{
    let machines = include_str!("../input.txt").lines().map(Machine::parse).collect::<Vec<Machine>>();
    println!("{}", machines.iter().map(|m| m.bfs_lights()).sum::<u32>());

    let mut sum  = 0;
    let mut vars = Vec::new();
    for m in machines.iter()
    {
        let mut matrix     = m.to_linear_system();
        let (pivots, free) = linear::echelon_form(&mut matrix);

        let mut min = i32::MAX;
        for assignment in counter::Counter::new(free.len(), *m.joltages.iter().max().unwrap())
        {
            vars.clear();
            vars.resize(m.buttons.len(), 0);
            for (&f, a) in free.iter().zip(assignment.into_iter())
            {
                vars[f] = a
            }

            if linear::back_substitute(&matrix, &pivots, &mut vars)
            {
                min = min.min(vars.iter().sum::<i32>())
            }
        }
        sum += min
    }
    println!("{sum}");
}

struct Machine
{
    lights:   Vec<bool>,
    buttons:  Vec<Vec<usize>>,
    joltages: Vec<i32>
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
            row.push(j);
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
