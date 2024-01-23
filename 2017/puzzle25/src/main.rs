use std::collections::HashMap;

fn main()
{
    let (steps, mut tm) = TM::parse(include_str!("../input.txt"));
    (0 .. steps).for_each(|_| tm.step());
    println!("{}", tm.checksum())
}

struct TM
{
    state: u8,
    left:  Vec<bool>,
    right: Vec<bool>,
    rules: HashMap<u8, Rule>
}

type Rule = ((bool, bool, u8), (bool, bool, u8));

impl TM
{
    fn parse(s : &str) -> (u32, TM)
    {
        let s          = s.strip_prefix("Begin in state ").unwrap();
        let state      = s.bytes().next().unwrap();
        let (_,     s) = s.split_once("Perform a diagnostic checksum after ").unwrap();
        let (steps, s) = s.split_once(' ').unwrap();
        let rules      = s.split("\n\n").skip(1).map(TM::parse_rule).collect();

        (steps.parse().unwrap(), TM { state, left: Vec::new(), right: Vec::new(), rules })
    }

    fn parse_rule(s : &str) -> (u8, Rule)
    {
        let s       = s.strip_prefix("In state ").unwrap();
        let state   = s.bytes().next().unwrap();
        let (_, s)  = s.split_once("Write the value ").unwrap();
        let a_write = s.strip_prefix('1').is_some();
        let (_, s)  = s.split_once("Move one slot to the ").unwrap();
        let a_left  = s.strip_prefix("left").is_some();
        let (_, s)  = s.split_once("Continue with state ").unwrap();
        let a_state = s.bytes().next().unwrap();
        let (_, s)  = s.split_once("Write the value ").unwrap();
        let b_write = s.strip_prefix('1').is_some();
        let (_, s)  = s.split_once("Move one slot to the ").unwrap();
        let b_left  = s.strip_prefix("left").is_some();
        let (_, s)  = s.split_once("Continue with state ").unwrap();
        let b_state = s.bytes().next().unwrap();

        (state, ((a_write, a_left, a_state), (b_write, b_left, b_state)))
    }

    fn step(&mut self)
    {
        let (zero, one)          = self.rules[&self.state];
        let (write, left, state) = if self.read() { one } else { zero };

        self.write(write);
        if left { self.move_left() } else { self.move_right() }
        self.state = state
    }

    fn read(&self) -> bool
    {
        self.right.last().copied().unwrap_or(false)
    }

    fn write(&mut self, bit : bool)
    {
        match self.right.last_mut()
        {
            None       => self.right.push(bit),
            Some(slot) => *slot = bit
        }
    }

    fn move_left(&mut self)
    {
        self.right.push(self.left.pop().unwrap_or(false))
    }

    fn move_right(&mut self)
    {
        self.left.push(self.right.pop().unwrap_or(false))
    }

    fn checksum(&self) -> u32
    {
        self.left.iter().chain(self.right.iter())
                 .map(|&bit| bit as u32)
                 .sum()
    }
}
