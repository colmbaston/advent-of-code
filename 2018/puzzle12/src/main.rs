use std::collections::HashSet;

fn main()
{
    let (mut state, rules) = State::parse(include_str!("../input.txt"));

    for _ in 1 ..= 20
    {
        state.next_generation(&rules);
    }

    // for part 2, assume it will fall into a linear series after some number of generations
    let mut prev_two = state.sum();
    state.next_generation(&rules);
    let mut prev_one = state.sum();

    // part 1: prev_two was the result after generation 20
    println!("{}", prev_two);

    // part 2: detect the linear series and compute the fifty-billionth term
    for g in 22 ..
    {
        state.next_generation(&rules);
        let current = state.sum();

        if current - prev_one == prev_one - prev_two
        {
            println!("{:?}", current + (50_000_000_000 - g) * (current - prev_one));
            break
        }

        prev_two = prev_one;
        prev_one = current;
    };

}

struct State
{
    index : i64,
    pots  : Vec<bool>
}

impl State
{
    fn parse(s : &str) -> (State, HashSet<u8>)
    {
        let mut ls        = s.lines();
        let initial_bytes = &ls.next().unwrap().as_bytes()[15..];
        ls.next();

        // always ensure the state is padded with four falses at each side
        // in case the rules "....# => #" or "#.... => #" are present
        let mut initial = Vec::with_capacity(initial_bytes.len() + 8);
        initial.resize(4, false);
        initial.extend(initial_bytes.iter().map(|&b| b == b'#'));
        initial.resize(initial.capacity(), false);

        let mut rules = HashSet::new();
        for rule in ls.map(|r| r.as_bytes())
        {
            if rule[9] == b'#'
            {
                rules.insert(rule[..5].iter().fold(0, |a, &x| a*2 + (x == b'#') as u8));
            }
        }

        (State { index: -4, pots: initial }, rules)
    }

    fn sum(&self) -> i64
    {
        (self.index ..).zip(self.pots.iter()).filter_map(|(i, &p)| if p { Some(i) } else { None }).sum()
    }

    fn next_generation(&mut self, rules : &HashSet<u8>)
    {
        let mut new = Vec::with_capacity(self.pots.len() + 4);
        new.resize(4, false);
        new.extend(self.pots.windows(5).map(|w| rules.contains(&w.iter().fold(0, |a, &x| a*2 + x as u8))));
        new.resize(new.capacity(), false);

        self.index -= 2;
        self.pots   = new;
    }
}
