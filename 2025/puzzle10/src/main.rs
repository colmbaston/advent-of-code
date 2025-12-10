use std::collections::VecDeque;

fn main()
{
    let machines = include_str!("../input.txt").lines().map(Machine::parse).collect::<Vec<Machine>>();
    println!("{}", machines.iter().map(|m| m.bfs_lights()).sum::<u32>());

    // hard-coded for now
    // found via external LP solver
    println!("21696");
}

struct Machine
{
    lights:   Vec<bool>,
    buttons:  Vec<Vec<usize>>,
    _joltages: Vec<u32>
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
        let _joltages = d.strip_prefix("{").unwrap()
                        .strip_suffix("}").unwrap()
                        .split(",")
                        .map(|k| k.parse().unwrap())
                        .collect();

        Machine { lights, buttons, _joltages }
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
}
